use terminator_dancer::{
    TerminatorRuntime, Transaction, Account, Instruction, InstructionData, AccountMeta, Pubkey,
    crypto::SolanaCrypto,
};
use std::collections::HashMap;
use ed25519_dalek::{SigningKey, Signer};
use rand::rngs::OsRng;

fn create_test_runtime() -> TerminatorRuntime {
    // Since runtime is async, we'll create it in each benchmark
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        TerminatorRuntime::new("nonexistent_config.toml").await.expect("Failed to create runtime")
    })
}

fn create_test_accounts(count: usize) -> HashMap<Pubkey, Account> {
    let mut accounts = HashMap::new();
    
    for _i in 0..count {
        let key = Pubkey::new_unique();
        
        let account = Account {
            lamports: 1_000_000_000, // 1 SOL
            data: vec![0u8; 128], // Small account data
            owner: [0u8; 32], // System program
            executable: false,
            rent_epoch: 0,
        };
        
        accounts.insert(key, account);
    }
    
    accounts
}

fn create_transfer_transactions(count: usize) -> Vec<Transaction> {
    let mut transactions = Vec::new();
    
    for _i in 0..count {
        let from = Pubkey::new_unique();
        let to = Pubkey::new_unique();
        
        let instruction = Instruction {
            program_id: Pubkey::system_program(),
            accounts: vec![
                AccountMeta {
                    pubkey: from,
                    is_signer: true,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: to,
                    is_signer: false,
                    is_writable: true,
                },
            ],
            data: InstructionData::Transfer { 
                from: from.0, 
                to: to.0, 
                lamports: 1000000, // 0.001 SOL
            },
        };
        
        transactions.push(Transaction {
            instructions: vec![instruction],
            payer: from.0,
            recent_blockhash: [1u8; 32],
            signatures: vec![[0u8; 64]],
        });
    }
    
    transactions
}

fn create_complex_transactions(count: usize) -> Vec<Transaction> {
    let mut transactions = Vec::new();
    
    for _i in 0..count {
        let payer = Pubkey::new_unique();
        let new_account = Pubkey::new_unique();
        let transfer_to = Pubkey::new_unique();
        
        // Multi-instruction transaction
        let instructions = vec![
            // Account creation
            Instruction {
                program_id: Pubkey::system_program(),
                accounts: vec![
                    AccountMeta {
                        pubkey: payer,
                        is_signer: true,
                        is_writable: true,
                    },
                    AccountMeta {
                        pubkey: new_account,
                        is_signer: true,
                        is_writable: true,
                    },
                ],
                data: InstructionData::CreateAccount {
                    from: payer.0,
                    to: new_account.0,
                    lamports: 1000000,
                    space: 128,
                    owner: [0u8; 32],
                },
            },
            // Transfer
            Instruction {
                program_id: Pubkey::system_program(),
                accounts: vec![
                    AccountMeta {
                        pubkey: payer,
                        is_signer: true,
                        is_writable: true,
                    },
                    AccountMeta {
                        pubkey: transfer_to,
                        is_signer: false,
                        is_writable: true,
                    },
                ],
                data: InstructionData::Transfer {
                    from: payer.0,
                    to: transfer_to.0,
                    lamports: 500000,
                },
            },
        ];
        
        transactions.push(Transaction {
            instructions,
            payer: payer.0,
            recent_blockhash: [1u8; 32],
            signatures: vec![[0u8; 64]],
        });
    }
    
    transactions
}

// Note: Due to the async nature of TerminatorRuntime, these benchmarks
// are simplified for demonstration. In a real benchmark setup, you'd
// want to use tokio-test or similar for async benchmarking.

fn main() {
    println!("Benchmark placeholder - tests would run here");
    println!("Note: Actual criterion benchmarks need async support");
    
    // Create a simple runtime test
    let _runtime = create_test_runtime();
    let _accounts = create_test_accounts(100);
    let _transactions = create_transfer_transactions(10);
    let _complex_transactions = create_complex_transactions(5);
    
    println!("Test data created successfully");
    
    // Test crypto operations
    let mut csprng = OsRng;
    let signing_key = SigningKey::generate(&mut csprng);
    let verifying_key = signing_key.verifying_key();
    let message = b"benchmark message for signature verification";
    let signature = signing_key.sign(message);
    
    let result = SolanaCrypto::verify_ed25519_signature(
        &signature.to_bytes(),
        message,
        &verifying_key.to_bytes(),
    );
    
    println!("Crypto test result: {:?}", result);
}

// Note: This would be the criterion benchmark setup, but we use a custom main instead
// use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
// 
// fn dummy_benchmark(c: &mut Criterion) {
//     c.bench_function("dummy", |b| b.iter(|| black_box(1 + 1)));
// }
// 
// criterion_group!(benches, dummy_benchmark);
// criterion_main!(benches); 