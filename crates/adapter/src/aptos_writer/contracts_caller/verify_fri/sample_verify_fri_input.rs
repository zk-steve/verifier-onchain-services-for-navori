use std::fs::File;
use std::io::BufReader;
use std::str::FromStr;

use aptos_sdk::move_types::u256::U256;
use aptos_sdk::move_types::value::MoveValue;

use crate::aptos_writer::contracts_caller::verify_fri::types::fri_verify_input::FriVerifyInput;

pub fn sample_verify_fri_input(
    index: isize,
) -> anyhow::Result<(
    FriVerifyInput,
    MoveValue,
    MoveValue,
    MoveValue,
    MoveValue,
    MoveValue,
)> {
    let file_path = format!(
        "./src/test_samples/test_samples/fri_verify/fri_verify_{}.json",
        index
    );
    let input_file = File::open(file_path)?;
    let reader = BufReader::new(input_file);
    let fri_verify_input: FriVerifyInput = serde_json::from_reader(reader)?;

    //proof
    let mut proof_vec = vec![];
    for i in 0..fri_verify_input.proof.len() {
        proof_vec.push(MoveValue::U256(U256::from_str(
            &fri_verify_input.proof[i].clone(),
        )?));
    }
    let proof = MoveValue::Vector(proof_vec);

    //queue
    let mut fri_queue_vec = vec![];
    for i in 0..fri_verify_input.fri_queue.len() {
        fri_queue_vec.push(MoveValue::U256(U256::from_str(
            &fri_verify_input.fri_queue[i].clone(),
        )?));
    }
    let fri_queue = MoveValue::Vector(fri_queue_vec);

    let evaluation_point =
        MoveValue::U256(U256::from_str(&fri_verify_input.evaluation_point.clone())?);
    let fri_step_size = MoveValue::U256(U256::from_str(&fri_verify_input.fri_step_size.clone())?);
    let expected_root = MoveValue::U256(U256::from_str(&fri_verify_input.expected_root.clone())?);
    Ok((
        fri_verify_input,
        proof,
        fri_queue,
        evaluation_point,
        fri_step_size,
        expected_root,
    ))
}
