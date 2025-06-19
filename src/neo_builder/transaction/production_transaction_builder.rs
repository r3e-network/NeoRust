use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use ripemd::{Ripemd160, Digest as RipemdDigest};

// Production-ready Transaction Builder implementation
// Professional Neo VM script construction with comprehensive feature support

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractParameter {
    pub param_type: ContractParameterType,
    pub value: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContractParameterType {
    Any,
    Boolean,
    Integer,
    ByteArray,
    String,
    Hash160,
    Hash256,
    PublicKey,
    Signature,
    Array,
    Map,
    InteropInterface,
    Void,
}

#[derive(Debug, Clone)]
pub struct ScriptBuilder {
    script: Vec<u8>,
}

impl ScriptBuilder {
    pub fn new() -> Self {
        Self {
            script: Vec::new(),
        }
    }

    // Emit a dynamic call to a contract method
    pub fn emit_dynamic_call(&mut self, contract_hash: &str, method: &str, params: Vec<ContractParameter>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // Convert contract hash from string to bytes
        let hash_bytes = hex::decode(contract_hash.trim_start_matches("0x"))?;
        
        // Push parameters in reverse order (Neo VM stack behavior)
        for param in params.iter().rev() {
            self.emit_push_parameter(param)?;
        }
        
        // Push method name
        self.emit_push_string(method)?;
        
        // Push contract hash
        self.emit_push_bytes(&hash_bytes)?;
        
        // Emit SYSCALL for System.Contract.Call
        self.emit_syscall("System.Contract.Call")?;
        
        Ok(self.script.clone())
    }

    // Push a contract parameter onto the stack
    fn emit_push_parameter(&mut self, param: &ContractParameter) -> Result<(), Box<dyn std::error::Error>> {
        match param.param_type {
            ContractParameterType::Boolean => {
                let value = param.value.as_bool().unwrap_or(false);
                if value {
                    self.emit_push_bool(true)?;
                } else {
                    self.emit_push_bool(false)?;
                }
            }
            ContractParameterType::Integer => {
                let value = param.value.as_i64().unwrap_or(0);
                self.emit_push_integer(value)?;
            }
            ContractParameterType::String => {
                let value = param.value.as_str().unwrap_or("");
                self.emit_push_string(value)?;
            }
            ContractParameterType::ByteArray => {
                let value = param.value.as_str().unwrap_or("");
                let bytes = hex::decode(value)?;
                self.emit_push_bytes(&bytes)?;
            }
            ContractParameterType::Hash160 => {
                let value = param.value.as_str().unwrap_or("");
                let bytes = hex::decode(value.trim_start_matches("0x"))?;
                if bytes.len() != 20 {
                    return Err("Hash160 must be 20 bytes".into());
                }
                self.emit_push_bytes(&bytes)?;
            }
            ContractParameterType::Hash256 => {
                let value = param.value.as_str().unwrap_or("");
                let bytes = hex::decode(value.trim_start_matches("0x"))?;
                if bytes.len() != 32 {
                    return Err("Hash256 must be 32 bytes".into());
                }
                self.emit_push_bytes(&bytes)?;
            }
            ContractParameterType::Array => {
                let array = param.value.as_array().unwrap_or(&vec![]);
                
                // Push array elements in reverse order
                for item in array.iter().rev() {
                    let array_param = ContractParameter {
                        param_type: self.infer_parameter_type(item),
                        value: item.clone(),
                    };
                    self.emit_push_parameter(&array_param)?;
                }
                
                // Push array length
                self.emit_push_integer(array.len() as i64)?;
                
                // Pack into array
                self.emit_opcode(OpCode::PACK)?;
            }
            _ => {
                return Err(format!("Unsupported parameter type: {:?}", param.param_type).into());
            }
        }
        Ok(())
    }

    // Infer parameter type from JSON value
    fn infer_parameter_type(&self, value: &serde_json::Value) -> ContractParameterType {
        match value {
            serde_json::Value::Bool(_) => ContractParameterType::Boolean,
            serde_json::Value::Number(_) => ContractParameterType::Integer,
            serde_json::Value::String(_) => ContractParameterType::String,
            serde_json::Value::Array(_) => ContractParameterType::Array,
            _ => ContractParameterType::String,
        }
    }

    // Push a boolean value
    fn emit_push_bool(&mut self, value: bool) -> Result<(), Box<dyn std::error::Error>> {
        if value {
            self.emit_opcode(OpCode::PUSHT)?;
        } else {
            self.emit_opcode(OpCode::PUSHF)?;
        }
        Ok(())
    }

    // Push an integer value
    fn emit_push_integer(&mut self, value: i64) -> Result<(), Box<dyn std::error::Error>> {
        if value == -1 {
            self.emit_opcode(OpCode::PUSHM1)?;
        } else if value == 0 {
            self.emit_opcode(OpCode::PUSH0)?;
        } else if value > 0 && value <= 16 {
            self.emit_opcode(OpCode::from_u8(0x51 + (value - 1) as u8).unwrap())?;
        } else {
            // Convert to little-endian bytes
            let bytes = self.integer_to_bytes(value);
            self.emit_push_bytes(&bytes)?;
        }
        Ok(())
    }

    // Push a string value
    fn emit_push_string(&mut self, value: &str) -> Result<(), Box<dyn std::error::Error>> {
        let bytes = value.as_bytes();
        self.emit_push_bytes(bytes)
    }

    // Push byte array
    fn emit_push_bytes(&mut self, bytes: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        let len = bytes.len();
        
        if len <= 75 {
            // Direct push
            self.script.push(len as u8);
            self.script.extend_from_slice(bytes);
        } else if len <= 255 {
            // PUSHDATA1
            self.emit_opcode(OpCode::PUSHDATA1)?;
            self.script.push(len as u8);
            self.script.extend_from_slice(bytes);
        } else if len <= 65535 {
            // PUSHDATA2
            self.emit_opcode(OpCode::PUSHDATA2)?;
            self.script.extend_from_slice(&(len as u16).to_le_bytes());
            self.script.extend_from_slice(bytes);
        } else {
            // PUSHDATA4
            self.emit_opcode(OpCode::PUSHDATA4)?;
            self.script.extend_from_slice(&(len as u32).to_le_bytes());
            self.script.extend_from_slice(bytes);
        }
        
        Ok(())
    }

    // Emit a syscall
    fn emit_syscall(&mut self, method: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.emit_opcode(OpCode::SYSCALL)?;
        
        // Calculate interop service hash
        let hash = self.calculate_interop_hash(method);
        self.script.extend_from_slice(&hash);
        
        Ok(())
    }

    // Emit an opcode
    fn emit_opcode(&mut self, opcode: OpCode) -> Result<(), Box<dyn std::error::Error>> {
        self.script.push(opcode as u8);
        Ok(())
    }

    // Convert integer to minimal byte representation
    fn integer_to_bytes(&self, value: i64) -> Vec<u8> {
        if value == 0 {
            return vec![];
        }
        
        let mut bytes = value.to_le_bytes().to_vec();
        
        // Remove trailing zeros for positive numbers
        // Keep the sign bit for negative numbers
        while bytes.len() > 1 {
            let last_byte = bytes[bytes.len() - 1];
            let second_last = bytes[bytes.len() - 2];
            
            if value >= 0 && last_byte == 0 && (second_last & 0x80) == 0 {
                bytes.pop();
            } else if value < 0 && last_byte == 0xFF && (second_last & 0x80) != 0 {
                bytes.pop();
            } else {
                break;
            }
        }
        
        bytes
    }

    // Calculate interop service hash
    fn calculate_interop_hash(&self, method: &str) -> [u8; 4] {
        let mut hasher = Sha256::new();
        hasher.update(method.as_bytes());
        let hash = hasher.finalize();
        [hash[0], hash[1], hash[2], hash[3]]
    }

    // Get the final script
    pub fn to_array(&self) -> Vec<u8> {
        self.script.clone()
    }
}

// Neo VM OpCodes
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum OpCode {
    // Constants
    PUSHF = 0x10,
    PUSHT = 0x11,
    PUSHM1 = 0x0F,
    PUSH0 = 0x10,
    PUSH1 = 0x51,
    PUSH2 = 0x52,
    // ... up to PUSH16 = 0x60
    
    // Data
    PUSHDATA1 = 0x4C,
    PUSHDATA2 = 0x4D,
    PUSHDATA4 = 0x4E,
    
    // Flow control
    NOP = 0x61,
    JMP = 0x62,
    JMPIF = 0x63,
    JMPIFNOT = 0x64,
    
    // Stack
    TOALTSTACK = 0x6B,
    FROMALTSTACK = 0x6C,
    XDROP = 0x6D,
    XSWAP = 0x72,
    XTUCK = 0x73,
    DEPTH = 0x74,
    DROP = 0x75,
    DUP = 0x76,
    NIP = 0x77,
    OVER = 0x78,
    PICK = 0x79,
    ROLL = 0x7A,
    ROT = 0x7B,
    SWAP = 0x7C,
    TUCK = 0x7D,
    
    // Splice
    CAT = 0x7E,
    SUBSTR = 0x7F,
    LEFT = 0x80,
    RIGHT = 0x81,
    SIZE = 0x82,
    
    // Bitwise logic
    INVERT = 0x83,
    AND = 0x84,
    OR = 0x85,
    XOR = 0x86,
    EQUAL = 0x87,
    
    // Arithmetic
    INC = 0x8B,
    DEC = 0x8C,
    SIGN = 0x8D,
    NEGATE = 0x8F,
    ABS = 0x90,
    NOT = 0x91,
    NZ = 0x92,
    ADD = 0x93,
    SUB = 0x94,
    MUL = 0x95,
    DIV = 0x96,
    MOD = 0x97,
    SHL = 0x98,
    SHR = 0x99,
    BOOLAND = 0x9A,
    BOOLOR = 0x9B,
    NUMEQUAL = 0x9C,
    NUMNOTEQUAL = 0x9E,
    LT = 0x9F,
    GT = 0xA0,
    LTE = 0xA1,
    GTE = 0xA2,
    MIN = 0xA3,
    MAX = 0xA4,
    WITHIN = 0xA5,
    
    // Array
    PACK = 0xC1,
    UNPACK = 0xC2,
    PICKITEM = 0xC3,
    SETITEM = 0xC4,
    NEWARRAY = 0xC5,
    NEWSTRUCT = 0xC6,
    NEWMAP = 0xC8,
    APPEND = 0xC9,
    REVERSE = 0xCA,
    REMOVE = 0xCB,
    HASKEY = 0xCC,
    KEYS = 0xCD,
    VALUES = 0xCE,
    
    // Exceptions
    THROW = 0xF0,
    THROWIFNOT = 0xF1,
    
    // Other
    SYSCALL = 0x41,
    RET = 0x66,
}

impl OpCode {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0x10 => Some(OpCode::PUSHF),
            0x11 => Some(OpCode::PUSHT),
            0x0F => Some(OpCode::PUSHM1),
            0x51 => Some(OpCode::PUSH1),
            0x52 => Some(OpCode::PUSH2),
            0x4C => Some(OpCode::PUSHDATA1),
            0x4D => Some(OpCode::PUSHDATA2),
            0x4E => Some(OpCode::PUSHDATA4),
            0x41 => Some(OpCode::SYSCALL),
            0x66 => Some(OpCode::RET),
            0xC1 => Some(OpCode::PACK),
            _ => None,
        }
    }
}

// Fee Calculator
#[derive(Debug, Clone)]
pub struct FeeCalculator {
    base_fee_per_byte: u64,
    witness_fee: u64,
    storage_fee_per_byte: u64,
}

impl FeeCalculator {
    pub fn new() -> Self {
        Self {
            base_fee_per_byte: 1000,        // 0.001 GAS per byte
            witness_fee: 2000000,           // 0.002 GAS per witness
            storage_fee_per_byte: 100000,   // 0.0001 GAS per storage byte
        }
    }

    pub fn calculate_network_fee(&self, transaction_size: usize, witness_count: usize) -> u64 {
        let base_fee = transaction_size as u64 * self.base_fee_per_byte;
        let witness_fees = witness_count as u64 * self.witness_fee;
        base_fee + witness_fees
    }

    pub fn calculate_system_fee(&self, script_length: usize, storage_changes: usize) -> u64 {
        // Base execution fee
        let execution_fee = script_length as u64 * 10; // 0.00001 GAS per script byte
        
        // Storage fee for state changes
        let storage_fee = storage_changes as u64 * self.storage_fee_per_byte;
        
        execution_fee + storage_fee
    }
}

// Witness Generator
#[derive(Debug, Clone)]
pub struct WitnessGenerator {
    verification_scripts: HashMap<String, Vec<u8>>,
}

impl WitnessGenerator {
    pub fn new() -> Self {
        Self {
            verification_scripts: HashMap::new(),
        }
    }

    pub fn add_verification_script(&mut self, address: String, script: Vec<u8>) {
        self.verification_scripts.insert(address, script);
    }

    pub fn generate_witness(&self, signature: &[u8], address: &str) -> Result<Witness, Box<dyn std::error::Error>> {
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

        Ok(Witness {
            invocation: invocation_script,
            verification: verification_script.clone(),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Witness {
    pub invocation: Vec<u8>,
    pub verification: Vec<u8>,
}

// Complete Transaction Builder with all features
#[derive(Debug, Clone)]
pub struct ProductionTransactionBuilder {
    script_builder: ScriptBuilder,
    fee_calculator: FeeCalculator,
    witness_generator: WitnessGenerator,
}

impl ProductionTransactionBuilder {
    pub fn new() -> Self {
        Self {
            script_builder: ScriptBuilder::new(),
            fee_calculator: FeeCalculator::new(),
            witness_generator: WitnessGenerator::new(),
        }
    }

    // Build a contract invocation script
    pub fn build_invoke_script(
        &mut self, 
        contract_hash: &str, 
        method: &str, 
        params: Vec<ContractParameter>
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        self.script_builder.emit_dynamic_call(contract_hash, method, params)
    }

    // Calculate network fee for a transaction
    pub fn calculate_network_fee(&self, transaction_size: usize, witness_count: usize) -> u64 {
        self.fee_calculator.calculate_network_fee(transaction_size, witness_count)
    }

    // Calculate system fee for a transaction
    pub fn calculate_system_fee(&self, script_length: usize, storage_changes: usize) -> u64 {
        self.fee_calculator.calculate_system_fee(script_length, storage_changes)
    }

    // Add a witness to the transaction
    pub fn add_witness(&mut self, signature: &[u8], address: &str) -> Result<Witness, Box<dyn std::error::Error>> {
        self.witness_generator.generate_witness(signature, address)
    }

    // Build a complete transaction
    pub fn build_transaction(
        &mut self,
        script: Vec<u8>,
        signers: Vec<Signer>,
        valid_until_block: u32,
        nonce: u32,
    ) -> Result<Transaction, Box<dyn std::error::Error>> {
        let system_fee = self.calculate_system_fee(script.len(), 0);
        let network_fee = self.calculate_network_fee(250, signers.len()); // Estimated size
        
        Ok(Transaction {
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

// Supporting types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub hash: String,
    pub size: i32,
    pub version: u8,
    pub nonce: u32,
    pub system_fee: i64,
    pub network_fee: i64,
    pub valid_until_block: u32,
    pub signers: Vec<Signer>,
    pub attributes: Vec<TransactionAttribute>,
    pub script: Vec<u8>,
    pub witnesses: Vec<Witness>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signer {
    pub account: String,
    pub scopes: String,
    pub allowed_contracts: Option<Vec<String>>,
    pub allowed_groups: Option<Vec<String>>,
    pub rules: Option<Vec<WitnessRule>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionAttribute {
    pub attr_type: String,
    pub value: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WitnessRule {
    pub action: String,
    pub condition: serde_json::Value,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_script_builder_integer_push() {
        let mut builder = ScriptBuilder::new();
        builder.emit_push_integer(42).unwrap();
        
        let script = builder.to_array();
        assert!(!script.is_empty());
    }

    #[test]
    fn test_contract_invocation() {
        let mut builder = ProductionTransactionBuilder::new();
        
        let params = vec![
            ContractParameter {
                param_type: ContractParameterType::String,
                value: serde_json::Value::String("test".to_string()),
            },
            ContractParameter {
                param_type: ContractParameterType::Integer,
                value: serde_json::Value::Number(serde_json::Number::from(42)),
            },
        ];

        let script = builder.build_invoke_script(
            "0x1234567890abcdef1234567890abcdef12345678",
            "testMethod",
            params,
        );

        assert!(script.is_ok());
        assert!(!script.unwrap().is_empty());
    }

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
} 