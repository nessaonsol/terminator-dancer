use crate::types::*;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub fn format_pubkey(pubkey: &Pubkey) -> String {
    format!("{:?}", pubkey.0)
}

pub fn log_transaction_result(result: &TransactionResult) {
    println!("Transaction Result:");
    println!("  Success: {}", result.success);
    println!("  Compute units consumed: {}", result.compute_units_consumed);
    println!("  Logs: {:?}", result.logs);
    if let Some(error) = &result.error {
        println!("  Error: {}", error);
    }
}
