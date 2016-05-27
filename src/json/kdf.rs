use serde::{Serialize, Serializer, Deserialize, Deserializer, Error as SerdeError};
use serde::de::Visitor;
use super::Error;

#[derive(Debug, PartialEq)]
pub enum Kdf {
	Pbkdf2,
	Scrypt,
}

impl Serialize for Kdf {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> 
	where S: Serializer {
		match *self {
			Kdf::Pbkdf2 => serializer.serialize_str("pbkdf2"),
			Kdf::Scrypt => serializer.serialize_str("scrypt"),
		}
	}
}

impl Deserialize for Kdf {
	fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
	where D: Deserializer {
		deserializer.deserialize(KdfVisitor)
	}
}

struct KdfVisitor;

impl Visitor for KdfVisitor {
	type Value = Kdf;

	fn visit_str<E>(&mut self, value: &str) -> Result<Self::Value, E> where E: SerdeError {
		match value {
			"pbkdf2" => Ok(Kdf::Pbkdf2),
			"scrypt" => Ok(Kdf::Scrypt),
			_ => Err(SerdeError::custom(Error::UnsupportedKdf))
		}
	}

	fn visit_string<E>(&mut self, value: String) -> Result<Self::Value, E> where E: SerdeError {
		self.visit_str(value.as_ref())
	}
}

