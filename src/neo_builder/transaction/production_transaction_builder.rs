//! Production-ready Transaction Builder utilities
//!
//! This module provides additional utilities for production transaction building,
//! including fee calculation and witness generation.
//!
//! For script building, use the canonical `ScriptBuilder` from
//! `neo_builder::script::ScriptBuilder`.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

// =============================================================================
// Fee Calculator
// =============================================================================

/// Calculator for Neo transaction fees.
///
/// Provides methods to estimate network and system fees for transactions.
#[derive(Debug, Clone)]
pub struct FeeCalculator {
    base_fee_per_byte: u64,
    witness_fee: u64,
    storage_fee_per_byte: u64,
}

impl Default for FeeCalculator {
    fn default() -> Self {
        Self::new()
    }
}

impl FeeCalculator {
    /// Creates a new FeeCalculator with default Neo N3 fee parameters.
    pub fn new() -> Self {
        Self {
            base_fee_per_byte: 1000,        // 0.001 GAS per byte
            witness_fee: 2000000,           // 0.002 GAS per witness
            storage_fee_per_byte: 100000,   // 0.0001 GAS per storage byte
        }
    }

    /// Creates a FeeCalculator with custom fee parameters.
    pub fn with_params(base_fee_per_byte: u64, witness_fee: u64, storage_fee_per_byte: u64) -> Self {
        Self {
            base_fee_per_byte,
            witness_fee,
            storage_fee_per_byte,
        }
    }

    /// Calculates the network fee for a transaction.
    ///
    /// # Arguments
    ///
    /// * `transaction_size` - The size of the transaction in bytes
    /// * `witness_count` - The number of witnesses in the transaction
    ///
    /// # Returns
    ///
    /// The estimated network fee in GAS (as integer, divide by 10^8 for decimal)
    pub fn calculate_network_fee(&self, transaction_size: usize, witness_count: usize) -> u64 {
        let base_fee = transaction_size as u64 * self.base_fee_per_byte;
        let witness_fees = witness_count as u64 * self.witness_fee;
        base_fee + witness_fees
    }

    /// Calculates the system fee for a transaction.
    ///
    /// # Arguments
    ///
    /// * `script_length` - The length of the transaction script in bytes
    /// * `storage_changes` - The number of storage changes
    ///
    /// # Returns
    ///
    /// The estimated system fee in GAS (as integer, divide by 10^8 for decimal)
    pub fn calculate_system_fee(&self, script_length: usize, storage_changes: usize) -> u64 {
        // Base execution fee
        let execution_fee = script_length as u64 * 10; // 0.00001 GAS per script byte

        // Storage fee for state changes
        let storage_fee = storage_changes as u64 * self.storage_fee_per_byte;

        execution_fee + storage_fee
    }
}

// =============================================================================
// Witness Generator
// =============================================================================

/// Generator for transaction witnesses.
///
/// Manages verification scripts and generates witnesses for transaction signing.
#[derive(Debug, Clone, Default)]
pub struct WitnessGenerator {
    verification_scripts: HashMap<String, Vec<u8>>,
}

impl WitnessGenerator {
    /// Creates a new empty WitnessGenerator.
    pub fn new() -> Self {
        Self {
            verification_scripts: HashMap::new(),
        }
    }

    /// Adds a verification script for an address.
    ///
    /// # Arguments
    ///
    /// * `address` - The Neo address
    /// * `script` - The verification script bytes
    pub fn add_verification_script(&mut self, address: String, script: Vec<u8>) {
        self.verification_scripts.insert(address, script);
    }

    /// Generates a witness for a signature.
    ///
    /// # Arguments
    ///
    /// * `signature` - The signature bytes
    /// * `address` - The address that signed
    ///
    /// # Returns
    ///
    /// A `Witness` containing the invocation and verification scripts
    pub fn generate_witness(&self, signature: &[u8], address: &str) -> Result<ProductionWitness, Box<dyn std::error::Error>> {
        let verification_script = self.verification_scripts
            .get(address)
            .ok_or("Verification script not found for address")?;

        // Create invocation script
        let mut invocation_script = Vec::new();

        // Push signature
        if signature.len() <= 75 {
            invocation_script.push(signature.len() as u8);
        } else {
            return Err("Signature too long".into());
        }
        invocation_script.extend_from_slice(signature);

        Ok(ProductionWitness {
            invocation: invocation_script,
            verification: verification_script.clone(),
        })
    }
}

// =============================================================================
// Supporting Types
// =============================================================================

/// A witness for a production transaction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionWitness {
    /// The invocation script (contains signatures)
    pub invocation: Vec<u8>,
    /// The verification script (contains public keys and verification logic)
    pub verification: Vec<u8>,
}

/// A signer for a production transaction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionSigner {
    /// The account script hash (as hex string)
    pub account: String,
    /// The witness scopes
    pub scopes: String,
    /// Allowed contracts (for CustomContracts scope)
    pub allowed_contracts: Option<Vec<String>>,
    /// Allowed groups (for CustomGroups scope)
    pub allowed_groups: Option<Vec<String>>,
    /// Witness rules
    pub rules: Option<Vec<ProductionWitnessRule>>,
}

/// A transaction attribute.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionTransactionAttribute {
    /// The attribute type
    pub attr_type: String,
    /// The attribute value
    pub value: serde_json::Value,
}

/// A witness rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionWitnessRule {
    /// The action (Allow or Deny)
    pub action: String,
    /// The condition
    pub condition: serde_json::Value,
}

/// A production transaction structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionTransaction {
    /// Transaction hash (calculated after signing)
    pub hash: String,
    /// Transaction size in bytes
    pub size: i32,
    /// Transaction version
    pub version: u8,
    /// Random nonce to prevent replay
    pub nonce: u32,
    /// System fee in GAS
    pub system_fee: i64,
    /// Network fee in GAS
    pub network_fee: i64,
    /// Block height until which the transaction is valid
    pub valid_until_block: u32,
    /// Transaction signers
    pub signers: Vec<ProductionSigner>,
    /// Transaction attributes
    pub attributes: Vec<ProductionTransactionAttribute>,
    /// Transaction script
    pub script: Vec<u8>,
    /// Transaction witnesses
    pub witnesses: Vec<ProductionWitness>,
}

// =============================================================================
// Production Transaction Builder
// =============================================================================

/// A production-ready transaction builder with fee calculation and witness generation.
///
/// This builder provides utilities for creating complete transactions with
/// proper fee estimation and witness management.
///
/// # Example
///
/// ```rust,ignore
/// use neo3::neo_builder::transaction::production_transaction_builder::*;
///
/// let mut builder = ProductionTransactionBuilder::new();
///
/// // Calculate fees
/// let network_fee = builder.calculate_network_fee(250, 1);
/// let system_fee = builder.calculate_system_fee(100, 0);
/// ```
#[derive(Debug, Clone)]
pub struct ProductionTransactionBuilder {
    fee_calculator: FeeCalculator,
    witness_generator: WitnessGenerator,
}

impl Default for ProductionTransactionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ProductionTransactionBuilder {
    /// Creates a new ProductionTransactionBuilder.
    pub fn new() -> Self {
        Self {
            fee_calculator: FeeCalculator::new(),
            witness_generator: WitnessGenerator::new(),
        }
    }

    /// Calculate network fee for a transaction.
    pub fn calculate_network_fee(&self, transaction_size: usize, witness_count: usize) -> u64 {
        self.fee_calculator.calculate_network_fee(transaction_size, witness_count)
    }

    /// Calculate system fee for a transaction.
    pub fn calculate_system_fee(&self, script_length: usize, storage_changes: usize) -> u64 {
        self.fee_calculator.calculate_system_fee(script_length, storage_changes)
    }

    /// Add a verification script for witness generation.
    pub fn add_verification_script(&mut self, address: String, script: Vec<u8>) {
        self.witness_generator.add_verification_script(address, script);
    }

    /// Generate a witness for a signature.
    pub fn generate_witness(&self, signature: &[u8], address: &str) -> Result<ProductionWitness, Box<dyn std::error::Error>> {
        self.witness_generator.generate_witness(signature, address)
    }

    /// Build a complete transaction structure.
    ///
    /// # Arguments
    ///
    /// * `script` - The transaction script
    /// * `signers` - The transaction signers
    /// * `valid_until_block` - The block height until which the transaction is valid
    /// * `nonce` - A random nonce to prevent replay attacks
    ///
    /// # Returns
    ///
    /// A `ProductionTransaction` with estimated fees
    pub fn build_transaction(
        &self,
        script: Vec<u8>,
        signers: Vec<ProductionSigner>,
        valid_until_block: u32,
        nonce: u32,
    ) -> Result<ProductionTransaction, Box<dyn std::error::Error>> {
        let system_fee = self.calculate_system_fee(script.len(), 0);
        let network_fee = self.calculate_network_fee(250, signers.len()); // Estimated size

        Ok(ProductionTransaction {
            hash: String::new(), // Will be calculated when signed
            size: 0,             // Will be calculated when serialized
            version: 0,
            nonce,
            system_fee: system_fee as i64,
            network_fee: network_fee as i64,
            valid_until_block,
            signers,
            attributes: vec![],
            script,
            witnesses: vec![],
        })
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fee_calculation() {
        let calculator = FeeCalculator::new();

        let network_fee = calculator.calculate_network_fee(250, 1);
        assert_eq!(network_fee, 250 * 1000 + 1 * 2000000); // base + witness fee

        let system_fee = calculator.calculate_system_fee(100, 0);
        assert_eq!(system_fee, 100 * 10); // execution fee only
    }

    #[test]
    fn test_witness_generation() {
        let mut generator = WitnessGenerator::new();

        // Add a verification script
        let verification_script = vec![0x41, 0x56, 0x9e, 0x7b, 0x41, 0x41, 0x68, 0x46, 0x45, 0x41];
        generator.add_verification_script("test_address".to_string(), verification_script.clone());

        // Generate witness
        let signature = vec![0x01, 0x02, 0x03]; // Mock signature
        let witness = generator.generate_witness(&signature, "test_address");

        assert!(witness.is_ok());
        let witness = witness.unwrap();
        assert_eq!(witness.verification, verification_script);
        assert_eq!(witness.invocation[0], 3); // Signature length
        assert_eq!(&witness.invocation[1..], &signature);
    }

    #[test]
    fn test_production_builder() {
        let builder = ProductionTransactionBuilder::new();

        let network_fee = builder.calculate_network_fee(250, 1);
        assert!(network_fee > 0);

        let system_fee = builder.calculate_system_fee(100, 0);
        assert!(system_fee > 0);
    }
}
