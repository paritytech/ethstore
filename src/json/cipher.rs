use serde::{Serialize, Serializer, Deserialize, Deserializer, Error as SerdeError};
use serde::de::Visitor;
use super::Error;

#[derive(Debug, PartialEq)]
pub enum Cipher {
	Aes128Ctr,
}

impl Serialize for Cipher {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> 
	where S: Serializer {
		match *self {
			Cipher::Aes128Ctr => serializer.serialize_str("aes-128-ctr"),
		}
	}
}

impl Deserialize for Cipher {
	fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
	where D: Deserializer {
		deserializer.deserialize(CipherVisitor)
	}
}

struct CipherVisitor;

impl Visitor for CipherVisitor {
	type Value = Cipher;

	fn visit_str<E>(&mut self, value: &str) -> Result<Self::Value, E> where E: SerdeError {
		match value {
			"aes-128-ctr" => Ok(Cipher::Aes128Ctr),
			_ => Err(SerdeError::custom(Error::UnsupportedCipher))
		}
	}

	fn visit_string<E>(&mut self, value: String) -> Result<Self::Value, E> where E: SerdeError {
		self.visit_str(value.as_ref())
	}
}

