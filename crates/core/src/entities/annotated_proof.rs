use std::collections::HashMap;

use ethers::types::U256;
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::entities::fri_statement::FRIMerkleStatement;
use crate::entities::gps_statement::MainProof;
use crate::entities::merkle_statement::MerkleStatement;

/// Adapted from https://github.com/zksecurity/stark-evm-adapter/blob/main/src/annotated_proof.rs
#[derive(Serialize, Deserialize, Debug, Clone)]
/// [AnnotatedProof] maps annotated proof json file which contains the original proof
/// and the annotations generated by verifier of stone-temp
pub struct AnnotatedProof {
    pub proof_hex: String,
    pub annotations: Vec<String>,
    pub extra_annotations: Vec<String>,
    pub proof_parameters: ProofParameters,
    pub public_input: PublicInput,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProofParameters {
    pub field: String,
    pub stark: StarkParameters,
    pub use_extension_field: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StarkParameters {
    pub fri: FriParameters,
    pub log_n_cosets: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FriParameters {
    pub fri_step_list: Vec<u32>,
    pub last_layer_degree_bound: u32,
    pub n_queries: u32,
    pub proof_of_work_bits: u32,
}

/// Public input for a cairo execution
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PublicInput {
    pub layout: String,
    pub memory_segments: HashMap<String, MemorySegment>,
    pub n_steps: u32,
    pub public_memory: Vec<PublicMemory>,
    pub rc_max: u32,
    pub rc_min: u32,
}

/// Memory segments for cairo builtins
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MemorySegment {
    pub begin_addr: u32,
    pub stop_ptr: u32,
}

/// Public memory for a cairo execution
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PublicMemory {
    pub address: u32,
    pub page: u32,
    // todo refactor to u256
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
/// [SplitProofs] maps the split proof json file which contains the main proof and the merkle statements
pub struct SplitProofs {
    pub main_proof: MainProof,
    pub merkle_statements: HashMap<String, MerkleStatement>,
    pub fri_merkle_statements: Vec<FRIMerkleStatement>,
}

impl AnnotatedProof {
    pub fn extract_interaction_elements(&self) -> (U256, U256) {
        let re = Regex::new(r"V->P: /cpu air/STARK/Interaction: Interaction element #\d+: Field Element\(0x([0-9a-f]+)\)").unwrap();
        let annotations = self.annotations.join("\n");

        let interaction_elements: Vec<U256> = re
            .captures_iter(&annotations)
            .filter_map(|cap| U256::from_str_radix(&cap[1], 16).ok())
            .collect();

        assert!(interaction_elements.len() == 3 || interaction_elements.len() == 6);

        (interaction_elements[0], interaction_elements[1])
    }
}
