use crate::{Result, TerminatorError};
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use sha2::{Sha256, Digest};
use blake3::Hasher as Blake3Hasher;

/// Real cryptographic operations using industry-standard libraries
pub struct SolanaCrypto;

impl SolanaCrypto {
    /// Verify an Ed25519 signature using the same crypto as Solana
    pub fn verify_ed25519_signature(
        signature_bytes: &[u8; 64],
        message: &[u8],
        public_key_bytes: &[u8; 32],
    ) -> Result<bool> {
        // Parse the public key
        let public_key = VerifyingKey::from_bytes(public_key_bytes)
            .map_err(|_| TerminatorError::InvalidSignature)?;
        
        // Parse the signature
        let signature = Signature::from_bytes(signature_bytes);
        
        // Verify the signature
        match public_key.verify(message, &signature) {
            Ok(()) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    /// Compute SHA256 hash exactly like Solana
    pub fn sha256_hash(data: &[u8]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.finalize().into()
    }

    /// Compute Blake3 hash
    pub fn blake3_hash(data: &[u8]) -> [u8; 32] {
        let mut hasher = Blake3Hasher::new();
        hasher.update(data);
        hasher.finalize().into()
    }

    /// Create a transaction message hash for signature verification
    pub fn create_transaction_message_hash(
        transaction_data: &[u8],
        recent_blockhash: &[u8; 32],
    ) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(b"solana-tx:");  // Solana message prefix
        hasher.update(transaction_data);
        hasher.update(recent_blockhash);
        hasher.finalize().into()
    }

    /// Validate a complete Solana transaction with real crypto
    pub fn validate_transaction_signatures(
        message_data: &[u8],
        signatures: &[&[u8; 64]],
        signers: &[&[u8; 32]],
        recent_blockhash: &[u8; 32],
    ) -> Result<bool> {
        if signatures.len() != signers.len() {
            return Err(TerminatorError::InvalidSignature);
        }

        // Create the message hash that was actually signed
        let message_hash = Self::create_transaction_message_hash(message_data, recent_blockhash);

        // Verify each signature
        for (signature, signer) in signatures.iter().zip(signers.iter()) {
            if !Self::verify_ed25519_signature(signature, &message_hash, signer)? {
                return Ok(false);
            }
        }

        Ok(true)
    }
}

/// Performance-oriented crypto operations
pub struct FastCrypto;

impl FastCrypto {
    /// Batch verify multiple signatures (more efficient for many signatures)
    pub fn batch_verify_signatures(
        signatures: &[&[u8; 64]], 
        messages: &[&[u8]], 
        public_keys: &[&[u8; 32]]
    ) -> Result<bool> {
        if signatures.len() != messages.len() || messages.len() != public_keys.len() {
            return Err(TerminatorError::InvalidSignature);
        }

        // For now, verify individually (real batch verification would be more complex)
        for ((signature, message), public_key) in signatures.iter()
            .zip(messages.iter())
            .zip(public_keys.iter()) {
            if !SolanaCrypto::verify_ed25519_signature(signature, message, public_key)? {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Optimized hash computation for high-throughput scenarios
    pub fn fast_hash_batch(messages: &[&[u8]]) -> Vec<[u8; 32]> {
        messages.iter().map(|msg| SolanaCrypto::sha256_hash(msg)).collect()
    }
}

/// Real Solana account address derivation
pub struct AddressDerivation;

impl AddressDerivation {
    /// Derive a Program Derived Address (PDA) exactly like Solana
    pub fn derive_program_address(
        seeds: &[&[u8]],
        program_id: &[u8; 32],
    ) -> Result<([u8; 32], u8)> {
        // Solana PDA derivation algorithm
        const PDA_MARKER: &[u8] = b"ProgramDerivedAddress";
        
        for bump in (0..=255u8).rev() {
            let mut hasher = Sha256::new();
            
            // Hash all seeds
            for seed in seeds {
                hasher.update(seed);
            }
            
            // Add bump seed
            hasher.update(&[bump]);
            
            // Add program ID
            hasher.update(program_id);
            
            // Add PDA marker
            hasher.update(PDA_MARKER);
            
            let hash: [u8; 32] = hasher.finalize().into();
            
            // Check if this is a valid PDA (not on the Ed25519 curve)
            if let Err(_) = VerifyingKey::from_bytes(&hash) {
                return Ok((hash, bump));
            }
        }
        
        Err(TerminatorError::ProgramError("Unable to find valid PDA".to_string()))
    }

    /// Find a Program Derived Address with a specific bump seed
    pub fn find_program_address(
        seeds: &[&[u8]],
        program_id: &[u8; 32],
    ) -> Result<[u8; 32]> {
        let (address, _bump) = Self::derive_program_address(seeds, program_id)?;
        Ok(address)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::OsRng;
    use ed25519_dalek::{SigningKey, Signer};

    #[test]
    fn test_real_signature_verification() {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        let verifying_key = signing_key.verifying_key();
        
        let message = b"test message for signature";
        let signature = signing_key.sign(message);
        
        let result = SolanaCrypto::verify_ed25519_signature(
            &signature.to_bytes(),
            message,
            &verifying_key.to_bytes(),
        ).unwrap();
        
        assert!(result, "Valid signature should verify");
    }

    #[test]
    fn test_invalid_signature() {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        let verifying_key = signing_key.verifying_key();
        
        let message = b"test message";
        let wrong_message = b"wrong message";
        let signature = signing_key.sign(message);
        
        let result = SolanaCrypto::verify_ed25519_signature(
            &signature.to_bytes(),
            wrong_message,
            &verifying_key.to_bytes(),
        ).unwrap();
        
        assert!(!result, "Invalid signature should not verify");
    }

    #[test]
    fn test_sha256_consistency() {
        let message = b"consistent hashing test";
        let hash1 = SolanaCrypto::sha256_hash(message);
        let hash2 = SolanaCrypto::sha256_hash(message);
        assert_eq!(hash1, hash2, "SHA256 should be deterministic");
    }

    #[test]
    fn test_program_derived_address() {
        let program_id = [1u8; 32];
        let seeds = [b"test", b"seed"];
        let seed_refs: Vec<&[u8]> = seeds.iter().map(|s| s.as_slice()).collect();
        
        let result = AddressDerivation::derive_program_address(&seed_refs, &program_id);
        assert!(result.is_ok(), "PDA derivation should succeed");
        
        let (address1, bump1) = result.unwrap();
        let (address2, bump2) = AddressDerivation::derive_program_address(&seed_refs, &program_id).unwrap();
        
        assert_eq!(address1, address2, "PDA derivation should be deterministic");
        assert_eq!(bump1, bump2, "Bump seed should be deterministic");
    }

    #[test]
    fn test_batch_verification() {
        let mut csprng = OsRng;
        let num_signatures = 5;
        
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
        
        let result = FastCrypto::batch_verify_signatures(&sig_refs, &msg_refs, &key_refs).unwrap();
        assert!(result, "Batch verification should succeed for valid signatures");
    }
} 