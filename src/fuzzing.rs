use crate::types::*;
use crate::{Result};

pub struct RuntimeFuzzer {
    pub iterations: usize,
}

impl RuntimeFuzzer {
    pub fn new(iterations: usize) -> Self {
        Self { iterations }
    }

    pub fn generate_random_transaction(&self) -> Transaction {
        let program_id = Pubkey::new_unique();
        let account = Pubkey::new_unique();
        
        let instruction = Instruction {
            program_id,
            accounts: vec![AccountMeta {
                pubkey: account,
                is_signer: true,
                is_writable: true,
            }],
            data: InstructionData::Generic {
                data: vec![1, 2, 3, 4],
            },
        };

        Transaction {
            instructions: vec![instruction],
            signatures: vec![[0u8; 64]],
            payer: account.0,
            recent_blockhash: [1u8; 32],
        }
    }

    pub fn run_fuzz_test<F>(&self, name: &str, test_fn: F)
    where
        F: Fn(&Transaction) -> Result<()>,
    {
        println!("Running fuzz test: {}", name);
        for _i in 0..self.iterations {
            let transaction = self.generate_random_transaction();
            if let Err(e) = test_fn(&transaction) {
                println!("Fuzz test failed: {}", e);
            }
        }
        println!("Completed {} iterations of {}", self.iterations, name);
    }
}
