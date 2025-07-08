use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use terminator_dancer::{
    TerminatorRuntime, Transaction, Account, Instruction, InstructionData,
    crypto::{SolanaCrypto, FastCrypto},
    solana_format::{SolanaTransactionParser, SolanaPubkey, SolanaHash},
};
use std::collections::HashMap;
use ed25519_dalek::{SigningKey, Signer};
use rand::rngs::OsRng;

fn create_test_runtime() -> TerminatorRuntime {
    TerminatorRuntime::new().expect("Failed to create runtime")
}

fn create_test_accounts(count: usize) -> HashMap<[u8; 32], Account> {
    let mut accounts = HashMap::new();
    
    for i in 0..count {
        let mut key = [0u8; 32];
        key[0..8].copy_from_slice(&(i as u64).to_le_bytes());
        
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
    
    for i in 0..count {
        let mut from = [0u8; 32];
        let mut to = [0u8; 32];
        from[0..8].copy_from_slice(&(i as u64).to_le_bytes());
        to[0..8].copy_from_slice(&((i + 1000) as u64).to_le_bytes());
        
        let instruction = Instruction {
            program_id: [0u8; 32], // System program
            accounts: vec![from, to],
            data: InstructionData::Transfer { 
                from, 
                to, 
                lamports: 1000000, // 0.001 SOL
            },
        };
        
        transactions.push(Transaction {
            instructions: vec![instruction],
            payer: from,
            recent_blockhash: [1u8; 32],
            signatures: vec![[0u8; 64]],
        });
    }
    
    transactions
}

fn create_complex_transactions(count: usize) -> Vec<Transaction> {
    let mut transactions = Vec::new();
    
    for i in 0..count {
        let mut payer = [0u8; 32];
        payer[0..8].copy_from_slice(&(i as u64).to_le_bytes());
        
        // Multi-instruction transaction
        let instructions = vec![
            // Account creation
            Instruction {
                program_id: [0u8; 32],
                accounts: vec![payer],
                data: InstructionData::CreateAccount {
                    from: payer,
                    to: [(i + 2000) as u8; 32],
                    lamports: 1000000,
                    space: 128,
                    owner: [0u8; 32],
                },
            },
            // Transfer
            Instruction {
                program_id: [0u8; 32],
                accounts: vec![payer, [(i + 3000) as u8; 32]],
                data: InstructionData::Transfer {
                    from: payer,
                    to: [(i + 3000) as u8; 32],
                    lamports: 500000,
                },
            },
        ];
        
        transactions.push(Transaction {
            instructions,
            payer,
            recent_blockhash: [1u8; 32],
            signatures: vec![[0u8; 64]],
        });
    }
    
    transactions
}

// Benchmark single transaction processing
fn bench_single_transaction_processing(c: &mut Criterion) {
    let mut group = c.benchmark_group("single_transaction");
    
    let runtime = create_test_runtime();
    let accounts = create_test_accounts(100);
    let transactions = create_transfer_transactions(1);
    
    group.bench_function("simple_transfer", |b| {
        b.iter(|| {
            let mut runtime_copy = runtime.clone();
            let mut accounts_copy = accounts.clone();
            runtime_copy.execute_transaction(
                black_box(&transactions[0]), 
                black_box(&mut accounts_copy)
            )
        })
    });
    
    group.finish();
}

// Benchmark batch transaction processing
fn bench_batch_transaction_processing(c: &mut Criterion) {
    let mut group = c.benchmark_group("batch_processing");
    group.throughput(Throughput::Elements(1000));
    
    let runtime = create_test_runtime();
    let accounts = create_test_accounts(2000);
    let transactions = create_transfer_transactions(1000);
    
    group.bench_function("1000_transfers", |b| {
        b.iter(|| {
            let mut runtime_copy = runtime.clone();
            let mut accounts_copy = accounts.clone();
            
            for transaction in &transactions {
                runtime_copy.execute_transaction(
                    black_box(transaction), 
                    black_box(&mut accounts_copy)
                ).unwrap();
            }
        })
    });
    
    group.finish();
}

// Benchmark complex multi-instruction transactions
fn bench_complex_transactions(c: &mut Criterion) {
    let mut group = c.benchmark_group("complex_transactions");
    
    let runtime = create_test_runtime();
    let accounts = create_test_accounts(5000);
    let transactions = create_complex_transactions(100);
    
    for tx_count in [10, 50, 100].iter() {
        group.throughput(Throughput::Elements(*tx_count));
        group.bench_with_input(
            BenchmarkId::new("multi_instruction", tx_count),
            tx_count,
            |b, &count| {
                b.iter(|| {
                    let mut runtime_copy = runtime.clone();
                    let mut accounts_copy = accounts.clone();
                    
                    for transaction in &transactions[0..count] {
                        runtime_copy.execute_transaction(
                            black_box(transaction), 
                            black_box(&mut accounts_copy)
                        ).unwrap();
                    }
                })
            }
        );
    }
    
    group.finish();
}

// Benchmark cryptographic operations
fn bench_crypto_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("crypto_operations");
    
    let mut csprng = OsRng;
    let signing_key = SigningKey::generate(&mut csprng);
    let verifying_key = signing_key.verifying_key();
    let message = b"benchmark message for signature verification";
    let signature = signing_key.sign(message);
    
    group.bench_function("signature_verification", |b| {
        b.iter(|| {
            SolanaCrypto::verify_ed25519_signature(
                black_box(&signature.to_bytes()),
                black_box(message),
                black_box(&verifying_key.to_bytes()),
            )
        })
    });
    
    group.bench_function("sha256_hash", |b| {
        b.iter(|| {
            SolanaCrypto::sha256_hash(black_box(message))
        })
    });
    
    // Batch signature verification
    let num_signatures = 100;
    let mut signatures = Vec::new();
    let mut messages = Vec::new();
    let mut public_keys = Vec::new();
    
    for i in 0..num_signatures {
        let signing_key = SigningKey::generate(&mut csprng);
        let verifying_key = signing_key.verifying_key();
        let message = format!("message {}", i);
        let signature = signing_key.sign(message.as_bytes());
        
        signatures.push(signature.to_bytes());
        messages.push(message.into_bytes());
        public_keys.push(verifying_key.to_bytes());
    }
    
    let sig_refs: Vec<&[u8; 64]> = signatures.iter().collect();
    let msg_refs: Vec<&[u8]> = messages.iter().map(|m| m.as_slice()).collect();
    let key_refs: Vec<&[u8; 32]> = public_keys.iter().collect();
    
    group.throughput(Throughput::Elements(num_signatures));
    group.bench_function("batch_signature_verification", |b| {
        b.iter(|| {
            FastCrypto::batch_verify_signatures(
                black_box(&sig_refs),
                black_box(&msg_refs),
                black_box(&key_refs),
            )
        })
    });
    
    group.finish();
}

// Benchmark Solana transaction format parsing
fn bench_solana_format(c: &mut Criterion) {
    let mut group = c.benchmark_group("solana_format");
    
    let from = SolanaPubkey::new([1u8; 32]);
    let to = SolanaPubkey::new([2u8; 32]);
    let blockhash = SolanaHash([3u8; 32]);
    
    let tx = SolanaTransactionParser::create_transfer_transaction(
        from, to, 1000000, blockhash
    );
    
    let serialized = SolanaTransactionParser::serialize_transaction(&tx).unwrap();
    let json = SolanaTransactionParser::transaction_to_json(&tx).unwrap();
    
    group.bench_function("serialize_transaction", |b| {
        b.iter(|| {
            SolanaTransactionParser::serialize_transaction(black_box(&tx))
        })
    });
    
    group.bench_function("deserialize_transaction", |b| {
        b.iter(|| {
            SolanaTransactionParser::parse_transaction(black_box(&serialized))
        })
    });
    
    group.bench_function("json_serialize", |b| {
        b.iter(|| {
            SolanaTransactionParser::transaction_to_json(black_box(&tx))
        })
    });
    
    group.bench_function("json_deserialize", |b| {
        b.iter(|| {
            SolanaTransactionParser::parse_transaction_json(black_box(&json))
        })
    });
    
    group.finish();
}

// Benchmark Program Derived Address operations
fn bench_pda_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("pda_operations");
    
    let program_id = [1u8; 32];
    let seeds = [b"benchmark", b"pda", b"test"];
    let seed_refs: Vec<&[u8]> = seeds.iter().map(|s| s.as_slice()).collect();
    
    group.bench_function("derive_program_address", |b| {
        b.iter(|| {
            terminator_dancer::crypto::AddressDerivation::derive_program_address(
                black_box(&seed_refs),
                black_box(&program_id),
            )
        })
    });
    
    group.finish();
}

// High-level throughput benchmark  
fn bench_runtime_throughput(c: &mut Criterion) {
    let mut group = c.benchmark_group("runtime_throughput");
    
    let runtime = create_test_runtime();
    let accounts = create_test_accounts(10000);
    
    // Test different batch sizes
    for batch_size in [100, 500, 1000, 2000].iter() {
        let transactions = create_transfer_transactions(*batch_size);
        group.throughput(Throughput::Elements(*batch_size as u64));
        
        group.bench_with_input(
            BenchmarkId::new("transactions_per_second", batch_size),
            batch_size,
            |b, &size| {
                b.iter(|| {
                    let mut runtime_copy = runtime.clone();
                    let mut accounts_copy = accounts.clone();
                    
                    let start = std::time::Instant::now();
                    
                    for transaction in &transactions[0..size] {
                        runtime_copy.execute_transaction(
                            black_box(transaction),
                            black_box(&mut accounts_copy)
                        ).unwrap();
                    }
                    
                    let elapsed = start.elapsed();
                    let tps = size as f64 / elapsed.as_secs_f64();
                    
                    // Print TPS for visibility
                    if size == 1000 {
                        println!("TPS: {:.0}", tps);
                    }
                })
            }
        );
    }
    
    group.finish();
}

criterion_group!(
    benches,
    bench_single_transaction_processing,
    bench_batch_transaction_processing,
    bench_complex_transactions,
    bench_crypto_operations,
    bench_solana_format,
    bench_pda_operations,
    bench_runtime_throughput,
);

criterion_main!(benches); 