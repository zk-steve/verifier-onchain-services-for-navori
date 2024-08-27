use std::fs::File;
use std::io::BufReader;

use crate::config::AppConfig;
use crate::contracts_caller::memory_page_fact_registry::register_continuous_page::register_continuous_page;
use crate::contracts_caller::memory_page_fact_registry::register_continuous_page_batch::register_continuous_page_batch;
use crate::contracts_caller::memory_page_fact_registry::types::memory_page_batch::MemoryPageEntries;
use crate::contracts_caller::memory_page_fact_registry::types::memory_page_fact_registry::RegisterMemoryPage;

pub async fn register_memory_batch(config: &AppConfig) -> anyhow::Result<bool> {
    let file_path =
        "src/data_samples/memory_page_fact_registry/register_continuous_page_batch.json"
            .to_string();
    let input_file = File::open(file_path)?;
    let reader = BufReader::new(input_file);
    let memory_page_entries: MemoryPageEntries = serde_json::from_reader(reader)?;

    register_continuous_page_batch(config, memory_page_entries).await
}

pub async fn register_memory(config: &AppConfig) -> anyhow::Result<bool> {
    let file_path =
        "src/data_samples/memory_page_fact_registry/register_memory_page.json".to_string();
    let input_file = File::open(file_path)?;
    let reader = BufReader::new(input_file);
    let memory_page_entries: RegisterMemoryPage = serde_json::from_reader(reader)?;

    register_continuous_page(config, memory_page_entries).await
}

pub async fn large_register_memory_batch(config: &AppConfig) -> anyhow::Result<bool> {
    let file_path =
        "src/data_samples/memory_page_fact_registry/large_data_register_continuous_page_batch.json"
            .to_string();
    let input_file = File::open(file_path)?;
    let reader = BufReader::new(input_file);
    let memory_page_entries: MemoryPageEntries = serde_json::from_reader(reader)?;

    register_continuous_page_batch(config, memory_page_entries).await
}
