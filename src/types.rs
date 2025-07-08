use serde::{Deserialize, Serialize};
use serde_with::{serde_as, Bytes};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Pubkey(pub [u8; 32]);

impl Pubkey {
    pub fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
    
    pub fn new_unique() -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let mut bytes = [0u8; 32];
        bytes[0..16].copy_from_slice(&nanos.to_le_bytes());
        Self(bytes)
    }

    // Common Solana program IDs
    pub fn system_program() -> Self {
        Self([0u8; 32])
    }

    pub fn token_program() -> Self {
        Self([
            6, 221, 246, 225, 215, 101, 161, 147, 217, 203, 225, 70, 206, 235, 121, 172,
            28, 180, 133, 237, 95, 91, 55, 145, 58, 140, 245, 133, 126, 255, 0, 169,
        ])
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub lamports: u64,
    pub data: Vec<u8>,
    pub owner: [u8; 32],
    pub executable: bool,
    pub rent_epoch: u64,
}

impl Account {
    pub fn new(lamports: u64, data: Vec<u8>, owner: [u8; 32]) -> Self {
        Self {
            lamports,
            data,
            owner,
            executable: false,
            rent_epoch: 0,
        }
    }

    pub fn new_executable(lamports: u64, data: Vec<u8>, owner: [u8; 32]) -> Self {
        Self {
            lamports,
            data,
            owner,
            executable: true,
            rent_epoch: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountMeta {
    pub pubkey: Pubkey,
    pub is_signer: bool,
    pub is_writable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instruction {
    pub program_id: Pubkey,
    pub accounts: Vec<AccountMeta>,
    pub data: InstructionData,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub instructions: Vec<Instruction>,
    #[serde_as(as = "Vec<Bytes>")]
    pub signatures: Vec<[u8; 64]>,
    #[serde_as(as = "Bytes")]
    pub payer: [u8; 32],
    #[serde_as(as = "Bytes")]
    pub recent_blockhash: [u8; 32],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InstructionData {
    Transfer {
        #[serde(with = "serde_bytes")]
        from: [u8; 32],
        #[serde(with = "serde_bytes")]
        to: [u8; 32],
        lamports: u64,
    },
    CreateAccount {
        #[serde(with = "serde_bytes")]
        from: [u8; 32],
        #[serde(with = "serde_bytes")]
        to: [u8; 32],
        lamports: u64,
        space: u64,
        #[serde(with = "serde_bytes")]
        owner: [u8; 32],
    },
    Assign {
        #[serde(with = "serde_bytes")]
        account: [u8; 32],
        #[serde(with = "serde_bytes")]
        owner: [u8; 32],
    },
    Generic {
        data: Vec<u8>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionResult {
    pub success: bool,
    pub compute_units_consumed: u64,
    pub logs: Vec<String>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeConfig {
    pub runtime: RuntimeSettings,
    pub bank: BankSettings,
    pub logging: LoggingSettings,
    pub performance: PerformanceSettings,
    pub networking: NetworkingSettings,
    pub testing: TestingSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeSettings {
    pub compute_budget: u64,
    pub max_transaction_size: usize,
    pub enable_fuzzing: bool,
    pub conformance_testing: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankSettings {
    pub initial_lamports: u64,
    pub rent_collection_enabled: bool,
    pub fee_rate_governor_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingSettings {
    pub level: String,
    pub enable_transaction_logs: bool,
    pub enable_execution_traces: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceSettings {
    pub max_concurrent_transactions: u32,
    pub cache_size_mb: u32,
    pub gc_threshold_mb: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkingSettings {
    pub max_connections: u32,
    pub connection_timeout_ms: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestingSettings {
    pub fuzz_iterations: u32,
    pub differential_testing: bool,
    pub property_testing: bool,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            runtime: RuntimeSettings {
                compute_budget: 1_400_000,
                max_transaction_size: 1232,
                enable_fuzzing: false,
                conformance_testing: false,
            },
            bank: BankSettings {
                initial_lamports: 1_000_000_000_000,
                rent_collection_enabled: true,
                fee_rate_governor_enabled: true,
            },
            logging: LoggingSettings {
                level: "info".to_string(),
                enable_transaction_logs: true,
                enable_execution_traces: true,
            },
            performance: PerformanceSettings {
                max_concurrent_transactions: 1000,
                cache_size_mb: 512,
                gc_threshold_mb: 1024,
            },
            networking: NetworkingSettings {
                max_connections: 1000,
                connection_timeout_ms: 5000,
            },
            testing: TestingSettings {
                fuzz_iterations: 1000,
                differential_testing: true,
                property_testing: true,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct BankState {
    pub accounts: HashMap<Pubkey, Account>,
    pub slot: u64,
    pub blockhash: [u8; 32],
    pub fee_calculator: FeeCalculator,
}

impl BankState {
    pub fn new() -> Self {
        Self {
            accounts: HashMap::new(),
            slot: 0,
            blockhash: [0u8; 32],
            fee_calculator: FeeCalculator::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FeeCalculator {
    pub lamports_per_signature: u64,
}

impl Default for FeeCalculator {
    fn default() -> Self {
        Self {
            lamports_per_signature: 5000,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionContext {
    pub compute_units_remaining: u64,
    pub log_messages: Vec<String>,
}

impl ExecutionContext {
    pub fn new(compute_budget: u64) -> Self {
        Self {
            compute_units_remaining: compute_budget,
            log_messages: Vec::new(),
        }
    }

    pub fn consume_compute_units(&mut self, units: u64) -> bool {
        if self.compute_units_remaining >= units {
            self.compute_units_remaining -= units;
            true
        } else {
            false
        }
    }

    pub fn log(&mut self, message: String) {
        self.log_messages.push(message);
    }
}
