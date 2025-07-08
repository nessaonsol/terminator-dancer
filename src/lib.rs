pub mod runtime;
pub mod bank;
pub mod executor;
pub mod conformance;
pub mod fuzzing;
pub mod types;
pub mod utils;
pub mod firedancer_integration;
pub mod crypto;
pub mod solana_format;

pub use runtime::TerminatorRuntime;
pub use bank::Bank;
pub use executor::TransactionExecutor;
pub use conformance::ConformanceHarness;
pub use fuzzing::RuntimeFuzzer;
pub use firedancer_integration::{FiredancerCrypto, FiredancerValidator, FiredancerConformanceTest};
pub use types::{Transaction, Account, Instruction, InstructionData, Pubkey, AccountMeta, TransactionResult};
pub use crypto::{SolanaCrypto, FastCrypto, AddressDerivation};
pub use solana_format::{SolanaTransaction, SolanaTransactionParser, SolanaPubkey, SolanaHash};

#[derive(Debug, thiserror::Error)]
pub enum TerminatorError {
    #[error("Transaction execution failed: {0}")]
    TransactionExecutionFailed(String),
    
    #[error("Account not found: {0}")]
    AccountNotFound(String),
    
    #[error("Insufficient funds")]
    InsufficientFunds,
    
    #[error("Invalid signature")]
    InvalidSignature,
    
    #[error("Program error: {0}")]
    ProgramError(String),
    
    #[error("Serialization error: {0}")]
    SerializationError(String),
    
    #[error("Conformance test failed: {0}")]
    ConformanceTestFailed(String),
}

pub type Result<T> = std::result::Result<T, TerminatorError>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::*;
    use crate::types::*;

    #[tokio::test]
    async fn test_runtime_initialization() {
        let runtime = TerminatorRuntime::new("nonexistent_config.toml").await;
        assert!(runtime.is_ok());
    }

    #[tokio::test]
    async fn test_transaction_execution() {
        let mut runtime = TerminatorRuntime::new("nonexistent_config.toml").await.unwrap();
        
        let program_id = Pubkey::new_unique();
        let account = Pubkey::new_unique();
        
        let instruction = Instruction {
            program_id,
            accounts: vec![AccountMeta {
                pubkey: account,
                is_signer: true,
                is_writable: true,
            }],
            data: InstructionData::Generic { data: vec![1, 2, 3, 4] },
        };

        let transaction = Transaction {
            instructions: vec![instruction],
            signatures: vec![[0u8; 64]],
            payer: account.0,
            recent_blockhash: [1u8; 32],
        };

        let result = runtime.execute_transaction(&transaction);
        assert!(result.is_ok());
        assert!(result.unwrap().success);
    }

    #[test]
    fn test_conformance_harness() {
        let mut harness = ConformanceHarness::new();
        
        harness.run_test("test_pass", || Ok(()));
        harness.run_test("test_fail", || Err(TerminatorError::TransactionExecutionFailed("test".to_string())));
        
        assert_eq!(harness.passed, 1);
        assert_eq!(harness.failed, 1);
    }

    #[test]
    fn test_fuzzer() {
        let fuzzer = RuntimeFuzzer::new(5);
        assert_eq!(fuzzer.iterations, 5);
        
        let transaction = fuzzer.generate_random_transaction();
        assert_eq!(transaction.instructions.len(), 1);
    }
}
