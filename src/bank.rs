use crate::{Result, TerminatorError};
use crate::types::*;
use std::collections::HashMap;

pub struct Bank {
    accounts: HashMap<Pubkey, u64>,
}

impl Bank {
    pub fn new() -> Self {
        Bank {
            accounts: HashMap::new(),
        }
    }

    pub fn process_transaction(&mut self, instruction: &Instruction) -> Result<()> {
        match instruction.program_id {
            _ => {
                let account = instruction.accounts.get(0).ok_or(TerminatorError::AccountNotFound("missing account".to_string()))?;
                let balance = self.accounts.entry(account.pubkey).or_insert(0);
                *balance += 10; // Simplified logic for demo purposes
            }
        }
        Ok(())
    }

    pub fn get_balance(&self, pubkey: &Pubkey) -> u64 {
        *self.accounts.get(pubkey).unwrap_or(&0)
    }
}
