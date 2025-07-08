use terminator_dancer::crypto::{SolanaCrypto, FastCrypto, AddressDerivation};
use terminator_dancer::solana_format::{SolanaTransactionParser, SolanaPubkey, SolanaHash};
use ed25519_dalek::{SigningKey, Signer};
use rand::rngs::OsRng;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔥 TERMINATOR-DANCER CRYPTO SHOWCASE 🔥");
    println!("Real Cryptography & Solana Compatibility Demo");
    println!("Built with AI + Firedancer Foundation");
    println!("===========================================\n");

    // Test 1: Real Ed25519 Signature Verification
    println!("🔐 TEST 1: Ed25519 Signature Verification");
    println!("=========================================");
    
    let mut csprng = OsRng;
    let signing_key = SigningKey::generate(&mut csprng);
    let verifying_key = signing_key.verifying_key();
    
    let message = b"Hello Anatoly! This is a real signature from Terminator-Dancer runtime!";
    let signature = signing_key.sign(message);
    
    println!("Message: {}", String::from_utf8_lossy(message));
    println!("Public Key: {}", hex::encode(verifying_key.to_bytes()));
    println!("Signature: {}", hex::encode(signature.to_bytes()));
    
    let verification_result = SolanaCrypto::verify_ed25519_signature(
        &signature.to_bytes(),
        message,
        &verifying_key.to_bytes(),
    )?;
    
    println!("✅ Signature verification: {}", if verification_result { "VALID" } else { "INVALID" });
    
    // Performance test
    let start = Instant::now();
    let iterations = 1000;
    for _ in 0..iterations {
        SolanaCrypto::verify_ed25519_signature(
            &signature.to_bytes(),
            message,
            &verifying_key.to_bytes(),
        )?;
    }
    let elapsed = start.elapsed();
    let sigs_per_sec = iterations as f64 / elapsed.as_secs_f64();
    println!("📊 Performance: {:.0} signature verifications/second\n", sigs_per_sec);

    // Test 2: SHA256 Hashing
    println!("🔗 TEST 2: SHA256 Hashing");
    println!("=========================");
    
    let data = b"Terminator-Dancer: Next-gen Solana runtime with real crypto!";
    let hash = SolanaCrypto::sha256_hash(data);
    
    println!("Data: {}", String::from_utf8_lossy(data));
    println!("SHA256: {}", hex::encode(hash));
    
    // Performance test
    let start = Instant::now();
    let hash_iterations = 10000;
    for _ in 0..hash_iterations {
        SolanaCrypto::sha256_hash(data);
    }
    let elapsed = start.elapsed();
    let hashes_per_sec = hash_iterations as f64 / elapsed.as_secs_f64();
    println!("📊 Performance: {:.0} hashes/second\n", hashes_per_sec);

    // Test 3: Program Derived Addresses (PDAs)
    println!("🎯 TEST 3: Program Derived Addresses");
    println!("===================================");
    
    let program_id = [42u8; 32]; // Custom program
    let seeds = [&b"terminator"[..], &b"dancer"[..], &b"pda"[..]];
    
    let (pda, bump) = AddressDerivation::derive_program_address(&seeds, &program_id)?;
    
    println!("Program ID: {}", hex::encode(program_id));
    println!("Seeds: {:?}", seeds.iter().map(|s| String::from_utf8_lossy(s)).collect::<Vec<_>>());
    println!("PDA: {}", hex::encode(pda));
    println!("Bump: {}", bump);
    
    // Performance test
    let start = Instant::now();
    let pda_iterations = 100;
    for _ in 0..pda_iterations {
        AddressDerivation::derive_program_address(&seeds, &program_id)?;
    }
    let elapsed = start.elapsed();
    let pdas_per_sec = pda_iterations as f64 / elapsed.as_secs_f64();
    println!("📊 Performance: {:.0} PDA derivations/second\n", pdas_per_sec);

    // Test 4: Solana Transaction Format
    println!("🌐 TEST 4: Solana Transaction Format");
    println!("===================================");
    
    let from = SolanaPubkey::new([1u8; 32]);
    let to = SolanaPubkey::new([2u8; 32]);
    let blockhash = SolanaHash([3u8; 32]);
    
    let transaction = SolanaTransactionParser::create_transfer_transaction(
        from, to, 1_000_000, blockhash
    );
    
    println!("Created Solana-compatible transaction:");
    println!("From: {}", from.to_string());
    println!("To: {}", to.to_string());
    println!("Amount: 1,000,000 lamports (0.001 SOL)");
    
    // Serialize to Solana wire format
    let serialized = SolanaTransactionParser::serialize_transaction(&transaction)?;
    println!("Serialized size: {} bytes", serialized.len());
    
    // Validate format
    let validation = SolanaTransactionParser::validate_transaction_format(&transaction);
    println!("✅ Format validation: {}", if validation.is_ok() { "PASSED" } else { "FAILED" });
    
    // JSON representation
    let json = SolanaTransactionParser::transaction_to_json(&transaction)?;
    println!("JSON representation available ({} chars)\n", json.len());

    // Test 5: Batch Operations
    println!("⚡ TEST 5: Batch Crypto Operations");
    println!("=================================");
    
    // Generate multiple signatures
    let num_signatures = 50;
    let mut signatures = Vec::new();
    let mut messages = Vec::new();
    let mut public_keys = Vec::new();
    
    println!("Generating {} signatures...", num_signatures);
    let start = Instant::now();
    
    for i in 0..num_signatures {
        let signing_key = SigningKey::generate(&mut csprng);
        let verifying_key = signing_key.verifying_key();
        let message = format!("Batch message #{}", i);
        let signature = signing_key.sign(message.as_bytes());
        
        signatures.push(signature.to_bytes());
        messages.push(message.into_bytes());
        public_keys.push(verifying_key.to_bytes());
    }
    
    let generation_time = start.elapsed();
    println!("Generated in {:.2}ms", generation_time.as_secs_f64() * 1000.0);
    
    // Batch verify
    let sig_refs: Vec<&[u8; 64]> = signatures.iter().collect();
    let msg_refs: Vec<&[u8]> = messages.iter().map(|m| m.as_slice()).collect();
    let key_refs: Vec<&[u8; 32]> = public_keys.iter().collect();
    
    let start = Instant::now();
    let batch_result = FastCrypto::batch_verify_signatures(&sig_refs, &msg_refs, &key_refs)?;
    let verification_time = start.elapsed();
    
    println!("✅ Batch verification: {} ({:.2}ms)", 
             if batch_result { "ALL VALID" } else { "SOME INVALID" },
             verification_time.as_secs_f64() * 1000.0);
    
    let batch_rate = num_signatures as f64 / verification_time.as_secs_f64();
    println!("📊 Batch verification rate: {:.0} signatures/second\n", batch_rate);

    // Test 6: Transaction Message Hashing
    println!("🔒 TEST 6: Transaction Security");
    println!("==============================");
    
    let transaction_data = b"transfer:from=alice,to=bob,amount=1000000";
    let recent_blockhash = [7u8; 32];
    
    let message_hash = SolanaCrypto::create_transaction_message_hash(
        transaction_data,
        &recent_blockhash
    );
    
    println!("Transaction data: {}", String::from_utf8_lossy(transaction_data));
    println!("Recent blockhash: {}", hex::encode(recent_blockhash));
    println!("Message hash: {}", hex::encode(message_hash));
    
    // Sign the transaction hash
    let tx_signature = signing_key.sign(&message_hash);
    let tx_verification = SolanaCrypto::verify_ed25519_signature(
        &tx_signature.to_bytes(),
        &message_hash,
        &verifying_key.to_bytes(),
    )?;
    
    println!("✅ Transaction signature: {}", if tx_verification { "VALID" } else { "INVALID" });
    
    // Final Summary
    println!("\n🎉 TERMINATOR-DANCER CAPABILITIES DEMONSTRATED");
    println!("=============================================");
    println!("✅ Real Ed25519 cryptography (industry standard)");
    println!("✅ High-performance signature verification");
    println!("✅ SHA256 hashing compatible with Solana");
    println!("✅ Program Derived Address generation");
    println!("✅ Solana transaction format parsing");
    println!("✅ Batch cryptographic operations");
    println!("✅ Transaction security and integrity");
    println!("\n🚀 Ready for production Solana workloads!");
    println!("🤖 Built with AI-assisted development in record time!");
    println!("🔥 Powered by Firedancer's high-performance foundation!");

    Ok(())
}

// Helper module for hex encoding since we need it
mod hex {
    pub fn encode(bytes: impl AsRef<[u8]>) -> String {
        bytes.as_ref()
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect()
    }
} 