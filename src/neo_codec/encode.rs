use std::fmt::Debug;

use crate::{
	codec::{CodecError, Decoder, Encoder},
	config::NeoConstants,
};
use primitive_types::{H160, H256};

pub trait NeoSerializable {
	type Error: Send + Sync + Debug;

	fn size(&self) -> usize;
	fn encode(&self, writer: &mut Encoder);
	fn decode(reader: &mut Decoder) -> Result<Self, Self::Error>
	where
		Self: Sized;
	fn to_array(&self) -> Vec<u8>;
}

impl NeoSerializable for H160 {
	type Error = CodecError;
	fn size(&self) -> usize {
		H160::len_bytes()
	}
	fn encode(&self, writer: &mut Encoder) {
		writer.write_bytes(self.as_fixed_bytes());
	}

	fn decode(reader: &mut Decoder) -> Result<Self, Self::Error>
	where
		Self: Sized,
	{
		let bytes = reader.read_bytes(NeoConstants::HASH160_SIZE as usize)?;
		<H160 as crate::neo_types::ScriptHashExtension>::from_slice(&bytes)
			.map_err(|_| CodecError::InvalidFormat)
	}

	fn to_array(&self) -> Vec<u8> {
		self.as_bytes().to_vec()
	}
}

impl NeoSerializable for H256 {
	type Error = CodecError;

	fn size(&self) -> usize {
		H256::len_bytes()
	}
	fn encode(&self, writer: &mut Encoder) {
		writer.write_bytes(&self.as_bytes());
	}

	fn decode(reader: &mut Decoder) -> Result<Self, CodecError>
	where
		Self: Sized,
	{
		let bytes = reader.read_bytes(NeoConstants::HASH256_SIZE as usize)?;
		if bytes.len() != 32 {
			return Err(CodecError::InvalidFormat);
		}
		let mut arr = [0u8; 32];
		arr.copy_from_slice(&bytes);
		Ok(H256(arr))
	}

	fn to_array(&self) -> Vec<u8> {
		self.as_bytes().to_vec()
	}
}

impl NeoSerializable for u8 {
	type Error = CodecError;

	fn size(&self) -> usize {
		1
	}
	fn encode(&self, writer: &mut Encoder) {
		writer.write_u8(*self);
	}

	fn decode(reader: &mut Decoder) -> Result<Self, CodecError>
	where
		Self: Sized,
	{
		Ok(reader.read_u8())
	}

	fn to_array(&self) -> Vec<u8> {
		vec![*self]
	}
}

pub trait VarSizeTrait {
	fn var_size(&self) -> usize;
	fn get_var_size(i: usize) -> usize {
		if i < 0xFD {
			1 // byte
		} else if i <= 0xFFFF {
			1 + 2 // 0xFD + uint16
		} else if i <= 0xFFFFFFFF {
			1 + 4 // 0xFE + uint32
		} else {
			1 + 8 // 0xFF + uint64
		}
	}
}

impl<T: NeoSerializable> VarSizeTrait for Vec<T> {
	fn var_size(&self) -> usize {
		let count_var_size = Self::get_var_size(self.len());
		count_var_size + self.iter().map(|item| item.size()).sum::<usize>()
	}
}

impl<T: NeoSerializable> VarSizeTrait for &[T] {
	fn var_size(&self) -> usize {
		let count_var_size = Self::get_var_size(self.len());
		count_var_size + self.iter().map(|item| item.size()).sum::<usize>()
	}
}

// fn var_size_of_serializables<T: NeoSerializable>(elements: &[T]) -> usize {
// 	let count_var_size = elements.len();
// 	count_var_size + elements.iter().map(|item| item.size()).sum::<usize>()
// }
