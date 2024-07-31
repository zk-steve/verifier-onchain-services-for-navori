use std::str::FromStr;

use aptos_sdk::move_types::identifier::Identifier;
use aptos_sdk::move_types::language_storage::ModuleId;
use aptos_sdk::move_types::value::serialize_values;
use aptos_sdk::rest_client::aptos_api_types::{Event, MoveType};
use aptos_sdk::types::transaction::{EntryFunction, TransactionPayload};
use crate::config::AppConfig;
use crate::contracts::helper::{build_transaction, get_event_from_transaction};
use crate::contracts::types::VerifyTransactionInput;

pub async fn verify_fri(config: &AppConfig, data: VerifyTransactionInput) -> anyhow::Result<(Event, Event)> {
    let payload = TransactionPayload::EntryFunction(
        EntryFunction::new(
            ModuleId::new(config.module_address, Identifier::new("fri_statement").unwrap()),
            Identifier::new("verify_fri").unwrap(),
            vec![],
            serialize_values(
                &vec![
                    data.proof,
                    data.fri_queue,
                    data.evaluation_point,
                    data.fri_step_size,
                    data.expected_root,
                ]
            ),
        ));
    let tx = build_transaction(payload, &config.account, config.chain_id);
    let transaction = config.client.submit_and_wait(&tx).await?.into_inner();
    let event_type = MoveType::from_str(&format!("{}::fri_statement::FriCtx", config.module_address)).unwrap();
    let fri_ctx_data = get_event_from_transaction(transaction.clone(), event_type).await?;
    let compute_next_layer_event_type = MoveType::from_str(&format!("{}::fri_statement::ComputeNextLayer", config.module_address)).unwrap();
    let compute_next_layer_data = get_event_from_transaction(
        transaction,
        compute_next_layer_event_type,
    ).await?;
    Ok((fri_ctx_data, compute_next_layer_data))
}