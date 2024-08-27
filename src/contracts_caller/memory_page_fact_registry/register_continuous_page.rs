use crate::config::AppConfig;
use crate::contracts_caller::memory_page_fact_registry::types::memory_page_fact_registry::RegisterMemoryPage;
use crate::contracts_caller::transaction_helper::build_transaction;
use aptos_sdk::move_types::identifier::Identifier;
use aptos_sdk::move_types::language_storage::ModuleId;
use aptos_sdk::move_types::u256::U256;
use aptos_sdk::move_types::value::{serialize_values, MoveValue};
use aptos_sdk::types::transaction::{EntryFunction, TransactionPayload};
use std::str::FromStr;

pub async fn register_continuous_page(
    config: &AppConfig,
    data: RegisterMemoryPage,
) -> anyhow::Result<bool> {
    let mut values = vec![];
    for e in &data.values {
        values.push(MoveValue::U256(U256::from_str(e)?))
    }

    let payload = TransactionPayload::EntryFunction(EntryFunction::new(
        ModuleId::new(
            config.module_address,
            Identifier::new("memory_page_fact_registry")?,
        ),
        Identifier::new("register_continuous_memorypage")?,
        vec![],
        serialize_values(&vec![
            MoveValue::U256(U256::from_str(&data.start_addr)?),
            MoveValue::Vector(values),
            MoveValue::U256(U256::from_str(&data.z)?),
            MoveValue::U256(U256::from_str(&data.alpha)?),
            MoveValue::U256(U256::from_str(&data.prime)?),
        ]),
    ));
    let tx = build_transaction(payload, &config.account, config.chain_id);
    let transaction = config.client.submit_and_wait(&tx).await?.into_inner();

    Ok(transaction.success())
}
