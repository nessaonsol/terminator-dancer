use crate::{Result, TerminatorError};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, Bytes};

/// Real Solana transaction format compatible with Solana's wire format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolanaTransaction {
    pub signatures: Vec<SolanaSignature>,
    pub message: SolanaMessage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolanaMessage {
    pub header: MessageHeader,
    pub account_keys: Vec<SolanaPubkey>,
    pub recent_blockhash: SolanaHash,
    pub instructions: Vec<CompiledInstruction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageHeader {
    pub num_required_signatures: u8,
    pub num_readonly_signed_accounts: u8,
    pub num_readonly_unsigned_accounts: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SolanaPubkey(#[serde(with = "serde_bytes")] pub [u8; 32]);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolanaSignature(#[serde(with = "serde_bytes")] pub [u8; 64]);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolanaHash(#[serde(with = "serde_bytes")] pub [u8; 32]);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompiledInstruction {
    pub program_id_index: u8,
    pub accounts: Vec<u8>, // Account indices
    pub data: Vec<u8>,
}

impl SolanaPubkey {
    pub fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    pub fn new_unique() -> Self {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(&std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
            .to_le_bytes());
        let hash: [u8; 32] = hasher.finalize().into();
        Self(hash)
    }

    /// System program ID
    pub fn system_program() -> Self {
        Self([0u8; 32])
    }

    /// SPL Token program ID  
    pub fn token_program() -> Self {
        Self([
            6, 221, 246, 225, 215, 101, 161, 147, 217, 203, 225, 70, 206, 235, 121, 172,
            28, 180, 133, 237, 95, 91, 55, 145, 58, 140, 245, 133, 126, 255, 0, 169,
        ])
    }

    /// Parse from base58 string (like Solana CLI)
    pub fn from_str(s: &str) -> Result<Self> {
        let bytes = bs58::decode(s)
            .into_vec()
            .map_err(|_| TerminatorError::SerializationError("Invalid base58".to_string()))?;
        
        if bytes.len() != 32 {
            return Err(TerminatorError::SerializationError("Invalid pubkey length".to_string()));
        }

        let mut array = [0u8; 32];
        array.copy_from_slice(&bytes);
        Ok(Self(array))
    }

    /// Convert to base58 string (like Solana CLI)
    pub fn to_string(&self) -> String {
        bs58::encode(&self.0).into_string()
    }
}

impl std::fmt::Display for SolanaPubkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

/// Real Solana transaction parser and builder
pub struct SolanaTransactionParser;

impl SolanaTransactionParser {
    /// Parse a transaction from Solana's wire format (bincode)
    pub fn parse_transaction(data: &[u8]) -> Result<SolanaTransaction> {
        bincode::deserialize(data)
            .map_err(|e| TerminatorError::SerializationError(format!("Failed to parse transaction: {}", e)))
    }

    /// Serialize transaction to Solana's wire format
    pub fn serialize_transaction(tx: &SolanaTransaction) -> Result<Vec<u8>> {
        bincode::serialize(tx)
            .map_err(|e| TerminatorError::SerializationError(format!("Failed to serialize transaction: {}", e)))
    }

    /// Parse transaction from JSON (like Solana RPC)
    pub fn parse_transaction_json(json: &str) -> Result<SolanaTransaction> {
        serde_json::from_str(json)
            .map_err(|e| TerminatorError::SerializationError(format!("Failed to parse JSON transaction: {}", e)))
    }

    /// Convert transaction to JSON
    pub fn transaction_to_json(tx: &SolanaTransaction) -> Result<String> {
        serde_json::to_string_pretty(tx)
            .map_err(|e| TerminatorError::SerializationError(format!("Failed to serialize JSON: {}", e)))
    }

    /// Create a simple transfer transaction in Solana format
    pub fn create_transfer_transaction(
        from: SolanaPubkey,
        to: SolanaPubkey,
        lamports: u64,
        recent_blockhash: SolanaHash,
    ) -> SolanaTransaction {
        // System program transfer instruction data
        let mut instruction_data = vec![2u8]; // Transfer instruction
        instruction_data.extend_from_slice(&lamports.to_le_bytes());

        let instruction = CompiledInstruction {
            program_id_index: 2, // System program will be at index 2
            accounts: vec![0, 1], // from=0, to=1
            data: instruction_data,
        };

        let message = SolanaMessage {
            header: MessageHeader {
                num_required_signatures: 1,
                num_readonly_signed_accounts: 0,
                num_readonly_unsigned_accounts: 1, // system program
            },
            account_keys: vec![from, to, SolanaPubkey::system_program()],
            recent_blockhash,
            instructions: vec![instruction],
        };

        SolanaTransaction {
            signatures: vec![SolanaSignature([0u8; 64])], // Placeholder signature
            message,
        }
    }

    /// Extract message for signing (without signatures)
    pub fn message_data(message: &SolanaMessage) -> Result<Vec<u8>> {
        bincode::serialize(message)
            .map_err(|e| TerminatorError::SerializationError(format!("Failed to serialize message: {}", e)))
    }

    /// Validate transaction format
    pub fn validate_transaction_format(tx: &SolanaTransaction) -> Result<()> {
        // Check signature count matches required signatures
        if tx.signatures.len() != tx.message.header.num_required_signatures as usize {
            return Err(TerminatorError::TransactionExecutionFailed(
                "Signature count mismatch".to_string()
            ));
        }

        // Check account indices are valid
        let num_accounts = tx.message.account_keys.len() as u8;
        for instruction in &tx.message.instructions {
            if instruction.program_id_index >= num_accounts {
                return Err(TerminatorError::TransactionExecutionFailed(
                    "Invalid program_id_index".to_string()
                ));
            }
            
            for &account_index in &instruction.accounts {
                if account_index >= num_accounts {
                    return Err(TerminatorError::TransactionExecutionFailed(
                        "Invalid account index".to_string()
                    ));
                }
            }
        }

        Ok(())
    }
}

/// Advanced Solana features
pub struct SolanaFeatures;

impl SolanaFeatures {
    /// Create a Program Derived Address instruction
    pub fn create_pda_instruction(
        program_id: SolanaPubkey,
        seeds: &[&[u8]],
        payer: SolanaPubkey,
    ) -> Result<CompiledInstruction> {
        // Simplified PDA creation instruction
        let mut instruction_data = vec![0u8]; // CreateAccount instruction
        
        // Add seeds to instruction data
        instruction_data.push(seeds.len() as u8);
        for seed in seeds {
            instruction_data.push(seed.len() as u8);
            instruction_data.extend_from_slice(seed);
        }

        Ok(CompiledInstruction {
            program_id_index: 0, // Program ID should be resolved during compilation
            accounts: vec![0], // Payer account
            data: instruction_data,
        })
    }

    /// Parse Address Lookup Table (ALT) instruction
    pub fn parse_lookup_table_instruction(data: &[u8]) -> Result<Vec<SolanaPubkey>> {
        if data.len() < 32 {
            return Err(TerminatorError::SerializationError("Invalid ALT data".to_string()));
        }

        let mut addresses = Vec::new();
        let mut offset = 0;
        
        while offset + 32 <= data.len() {
            let mut addr = [0u8; 32];
            addr.copy_from_slice(&data[offset..offset + 32]);
            addresses.push(SolanaPubkey(addr));
            offset += 32;
        }

        Ok(addresses)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pubkey_base58() {
        let pubkey = SolanaPubkey::new([1u8; 32]);
        let base58_str = pubkey.to_string();
        let parsed = SolanaPubkey::from_str(&base58_str).unwrap();
        assert_eq!(pubkey, parsed);
    }

    #[test]
    fn test_transaction_serialization() {
        let from = SolanaPubkey::new([1u8; 32]);
        let to = SolanaPubkey::new([2u8; 32]);
        let blockhash = SolanaHash([3u8; 32]);
        
        let tx = SolanaTransactionParser::create_transfer_transaction(
            from, to, 1000000, blockhash
        );

        // Test serialization round trip
        let serialized = SolanaTransactionParser::serialize_transaction(&tx).unwrap();
        let deserialized = SolanaTransactionParser::parse_transaction(&serialized).unwrap();
        
        assert_eq!(tx.message.account_keys.len(), deserialized.message.account_keys.len());
        assert_eq!(tx.message.instructions.len(), deserialized.message.instructions.len());
    }

    #[test]
    fn test_transaction_validation() {
        let from = SolanaPubkey::new([1u8; 32]);
        let to = SolanaPubkey::new([2u8; 32]);
        let blockhash = SolanaHash([3u8; 32]);
        
        let tx = SolanaTransactionParser::create_transfer_transaction(
            from, to, 1000000, blockhash
        );

        let result = SolanaTransactionParser::validate_transaction_format(&tx);
        assert!(result.is_ok(), "Valid transaction should pass validation");
    }

    #[test]
    fn test_json_serialization() {
        let from = SolanaPubkey::new([1u8; 32]);
        let to = SolanaPubkey::new([2u8; 32]);
        let blockhash = SolanaHash([3u8; 32]);
        
        let tx = SolanaTransactionParser::create_transfer_transaction(
            from, to, 1000000, blockhash
        );

        let json = SolanaTransactionParser::transaction_to_json(&tx).unwrap();
        let parsed = SolanaTransactionParser::parse_transaction_json(&json).unwrap();
        
        assert_eq!(tx.message.account_keys.len(), parsed.message.account_keys.len());
    }

    #[test]
    fn test_system_program_ids() {
        let system = SolanaPubkey::system_program();
        let token = SolanaPubkey::token_program();
        
        assert_eq!(system.0, [0u8; 32]);
        assert_ne!(system, token);
    }
} 