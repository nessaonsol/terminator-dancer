use crate::types::*;
use crate::{Result};

pub struct TransactionExecutor {
    pub compute_budget: u64,
}

impl TransactionExecutor {
    pub fn new(compute_budget: u64) -> Self {
        Self { compute_budget }
    }

    pub fn execute(&self, _transaction: &Transaction) -> Result<TransactionResult> {
        Ok(TransactionResult {
            success: true,
            compute_units_consumed: 100,
            logs: vec!["Transaction executed successfully".to_string()],
            error: None,
        })
    }
}
