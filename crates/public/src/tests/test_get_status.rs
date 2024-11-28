use reqwest::Client;
use serde_json::{json, Value};
use tokio;
use tokio_postgres::NoTls;

#[tokio::test]
async fn test_get_status() {
    let client = Client::new();

    // Set up the database
    setup_database().await;
    println!("✅ Database setup completed");

    test_failed(client.clone()).await;
    println!("✅ test_failed completed");

    test_invalid(client.clone()).await;
    println!("✅ test_invalid completed");

    test_unknown(client.clone()).await;
    println!("✅ test_unknown completed");

    test_in_progress(client.clone()).await;
    println!("✅ test_in_progress completed");

    test_not_created(client.clone()).await;
    println!("✅ test_not_created completed");

    test_processed(client.clone()).await;
    println!("✅ test_processed completed");

    test_onchain(client.clone()).await;
    println!("✅ test_onchain completed");
}

async fn test_failed(client: Client) {
    let customer_id = "93bc3373-5115-4f99-aecc-1bc57997bfd3".to_string();
    let cairo_job_key = "11395dd2-b874-4c11-8744-ba6482da997d".to_string();

    let expected =json!(
        {
            "status" : "FAILED",
            "error_log": "Sharp task failed",
        }
    );
    let res = get_response(client, customer_id, cairo_job_key).await;
    assert_eq!(res, expected, "Response did not match expected value");
}

async fn test_invalid(client: Client) {
    let customer_id = "18dc4b30-8b46-42d1-8b51-aba8c8abc7b0".to_string();
    let cairo_job_key = "09a10775-7294-4e5d-abbc-7659caa1a330".to_string();

    let expected =json!(
        {
            "status" : "INVALID",
            "invalid_reason": "INVALID_CAIRO_PIE_FILE_FORMAT",
            "error_log": "The Cairo PIE file has a wrong format. \
                        Deserialization ended with \
                        exception: Invalid prefix for zip file..",
        }
    );
    let res = get_response(client, customer_id, cairo_job_key).await;
    assert_eq!(res, expected, "Response did not match expected value");
}

async fn test_unknown(client: Client) {
    let customer_id = "2dd71442-58ca-4c35-a6de-8e637ff3c24b".to_string();
    let cairo_job_key = "f946ec7d-c3bf-42df-8bf0-9bcc751a8b3e".to_string();

    let expected =json!(
        {
            "status" : "FAILED",
        }
    );
    let res = get_response(client, customer_id, cairo_job_key).await;
    assert_eq!(res, expected, "Response did not match expected value");
}

async fn test_in_progress(client: Client) {
    let customer_id = "e703be2c-9ffe-4992-b968-da75da75d0b8".to_string();
    let cairo_job_key = "37e9d193-8e94-4df3-893a-cafa62a418c0".to_string();

    let expected =json!(
        {
            "status" : "IN_PROGRESS",
            "validation_done": false
        }
    );
    let res = get_response(client, customer_id, cairo_job_key).await;
    assert_eq!(res, expected, "Response did not match expected value");
}

async fn test_not_created(client: Client) {
    let customer_id = "040832f8-245f-4f05-a165-e2810e30047f".to_string();
    let cairo_job_key = "803eac13-3dbb-4ad2-a1df-311cfc2829cf".to_string();

    let expected =json!(
        {
            "status" : "NOT_CREATED",
            "validation_done": false
        }
    );
    let res = get_response(client, customer_id, cairo_job_key).await;
    assert_eq!(res, expected, "Response did not match expected value");
}

async fn test_processed(client: Client) {
    let customer_id = "8758d917-bbdc-4573-97ae-817e94fa31fb".to_string();
    let cairo_job_key = "59732e57-5722-4eb7-98db-8b90b89276f8".to_string();

    let expected =json!(
        {
            "status" : "PROCESSED",
            "validation_done": false
        }
    );
    let res = get_response(client, customer_id, cairo_job_key).await;
    assert_eq!(res, expected, "Response did not match expected value");
}

async fn test_onchain(client: Client) {
    let customer_id = "e3133ecb-e6e9-493a-ad64-ab9a4495af57".to_string();
    let cairo_job_key = "39af2c49-0c81-450e-91a9-aeff8dba2318".to_string();

    let expected =json!(
        {
            "status" : "ONCHAIN",
            "validation_done": true
        }
    );
    let res = get_response(client, customer_id, cairo_job_key).await;
    assert_eq!(res, expected, "Response did not match expected value");
}


async fn get_response(client: Client, customer_id: String, cairo_job_key: String) -> Value {
    let url = format!("http://localhost:8000/v1/gateway/get_status?customer_id={}&cairo_job_key={}",
        customer_id, cairo_job_key
    );
    client
        .get(&url)
        .send()
        .await
        .expect("Failed to send GET request")
        .json::<Value>()
        .await
        .expect("Failed to parse response body as JSON")
}

async fn setup_database() {
    let (client, connection) = tokio_postgres::connect("postgres://postgres:changeme@10.20.10.122:5432/postgres", NoTls)
        .await
        .expect("Failed to connect to database");

    // Spawn the connection in the background
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    // SQL to drop and recreate the table
    let reset_queries = r#"
        DROP TABLE IF EXISTS jobs;

        CREATE TABLE jobs (
            id UUID PRIMARY KEY,
            customer_id VARCHAR NOT NULL,
            cairo_job_key VARCHAR NOT NULL,
            status VARCHAR NOT NULL,
            validation_done BOOLEAN NOT NULL,
            created_on TIMESTAMP NOT NULL DEFAULT NOW(),
            updated_on TIMESTAMP NOT NULL DEFAULT NOW()
        );

        INSERT INTO jobs (id, customer_id, cairo_job_key, status, validation_done)
        VALUES
        ('2a3ee88d-e19d-43ed-a79e-da9a28dc9525', '93bc3373-5115-4f99-aecc-1bc57997bfd3', '11395dd2-b874-4c11-8744-ba6482da997d', 'FAILED', false),

        ('58f667ea-67b3-4b32-b4f8-ef24ea1c8f12', '18dc4b30-8b46-42d1-8b51-aba8c8abc7b0', '09a10775-7294-4e5d-abbc-7659caa1a330', 'INVALID', false),

        ('f2c604b7-52c5-4b69-9a67-de1276f9b8f8', '2dd71442-58ca-4c35-a6de-8e637ff3c24b', 'f946ec7d-c3bf-42df-8bf0-9bcc751a8b3e', 'UNKNOWN', false),

        ('d7045419-2b0f-4210-9e3d-7fb002839202', 'e703be2c-9ffe-4992-b968-da75da75d0b8', '37e9d193-8e94-4df3-893a-cafa62a418c0', 'IN_PROGRESS', false),

        ('549139a0-b288-401c-afb4-0f1018fd99f8', '040832f8-245f-4f05-a165-e2810e30047f', '803eac13-3dbb-4ad2-a1df-311cfc2829cf', 'NOT_CREATED', false),

        ('2283042d-f102-4ee6-a92f-73f3a86850e8', '8758d917-bbdc-4573-97ae-817e94fa31fb', '59732e57-5722-4eb7-98db-8b90b89276f8', 'PROCESSED', false),

        ('69f7ae7a-e981-44d2-9eb2-dfa551474870', 'e3133ecb-e6e9-493a-ad64-ab9a4495af57', '39af2c49-0c81-450e-91a9-aeff8dba2318', 'ONCHAIN', true);
    "#;

    client
        .batch_execute(reset_queries)
        .await
        .expect("Failed to reset database");
}