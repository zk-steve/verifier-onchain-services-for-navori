mod options;

use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use clap::{Parser, Subcommand};
use deadpool_diesel::postgres::Pool;
use deadpool_diesel::{Manager, Runtime};
use graphile_worker::WorkerOptions;
use irelia_adapter::aptos_writer::config::{AppConfig, Config};
use irelia_adapter::repositories::postgres::job_db::JobDBRepository;
use irelia_adapter::worker::WorkerAdapter;
use irelia_common::cli_args::CliArgs;
use irelia_common::kill_signals;
use irelia_common::loggers::telemetry::init_telemetry;
use irelia_core::ports::worker::WorkerPort;
use irelia_worker::app_state::State;
use irelia_worker::jobs::job_worker::JobWorker;
use irelia_worker::jobs::merkle_statement_job::MerkleStatementJob;
use irelia_worker::jobs::register_continuous_page_job::RegisterContinuousJob;
use irelia_worker::jobs::verify_fri_job::VerifyFriJob;
use irelia_worker::jobs::verify_proof_and_register_job::VerifyAndRegisterJob;
use irelia_worker::router::routes;
use opentelemetry::global;
use sqlx::postgres::PgConnectOptions;
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;
use tracing::info;

use crate::options::Options;

#[tokio::main]
async fn main() {
    let options: Options = CliArgs::default_run_or_get_options(env!("APP_VERSION"));
    init_telemetry(
        options.service_name.as_str(),
        options.exporter_endpoint.as_str(),
        options.log.level.as_str(),
    );

    let server = tokio::spawn(serve(options.clone()));

    run_workers(options).await;

    // Wait for the server to finish shutting down
    tokio::try_join!(server).expect("Failed to run server");

    global::shutdown_tracer_provider();
    info!("Shutdown successfully!");
}

/// Irelia Worker.
#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
    /// Config file
    #[arg(short, long, default_value = "config/00-default.toml")]
    config_path: Vec<String>,
    /// Print version
    #[clap(short, long)]
    version: bool,
}

#[derive(Subcommand, Clone, Debug)]
enum Commands {
    /// Print config
    Config,
}

pub async fn serve(options: Options) {
    let routes = routes().layer((
        TraceLayer::new_for_http(),
        // Graceful shutdown will wait for outstanding requests to complete. Add a timeout so
        // requests don't hang forever.
        TimeoutLayer::new(Duration::from_secs(10)),
    ));

    let endpoint = format!("{}:{}", options.server.url.as_str(), options.server.port);
    let listener = tokio::net::TcpListener::bind(endpoint.clone())
        .await
        .unwrap();
    info!("listening on http://{}", endpoint);
    axum::serve(listener, routes)
        .with_graceful_shutdown(kill_signals::wait_for_kill_signals())
        .await
        .unwrap();
}

pub async fn run_workers(options: Options) {
    info!("Using postgres database: {}", &options.pg.url);
    let manager = Manager::new(&options.pg.url, Runtime::Tokio1);
    let pool = Pool::builder(manager)
        .max_size(options.pg.max_size.try_into().unwrap())
        .build()
        .unwrap();

    let job_port = Arc::new(JobDBRepository::new(pool.clone()));

    let app_config = AppConfig::from(Config {
        node_url: options.verifier.aptos_node_url.clone(),
        private_key: options.verifier.aptos_private_key.clone(),
        chain_id: options.verifier.aptos_chain_id.clone(),
        aptos_verifier_address: options.verifier.aptos_verifier_contract_address.clone(),
    });

    let sequence_number = app_config
        .client
        .get_account(app_config.account.address())
        .await
        .unwrap()
        .into_inner()
        .sequence_number;
    app_config.account.set_sequence_number(sequence_number);

    let worker_adapter: Arc<dyn WorkerPort + Send + Sync> = Arc::new(
        WorkerAdapter::new(
            &options.pg.url,
            options.pg.max_size,
            options.worker.schema.clone(),
        )
        .await,
    );

    let state = State::new(job_port, worker_adapter, app_config);

    let pg_options = PgConnectOptions::from_str(&options.pg.url).unwrap();
    let pg_pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(options.pg.max_size)
        .connect_with(pg_options)
        .await
        .unwrap();

    let worker = WorkerOptions::default()
        .concurrency(options.worker.concurrent)
        .schema(options.worker.schema.as_str())
        .add_extension(state)
        .define_job::<JobWorker>()
        .define_job::<MerkleStatementJob>()
        .define_job::<VerifyFriJob>()
        .define_job::<RegisterContinuousJob>()
        .define_job::<VerifyAndRegisterJob>()
        .pg_pool(pg_pool)
        .init()
        .await
        .unwrap();

    worker.run().await.unwrap();
}
