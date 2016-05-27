use rustc_serialize::hex::{FromHex, ToHex};
use serde::{Serialize, Serializer, Deserialize, Deserializer, Error as SerdeError};
use serde::de::Visitor;
use serde_json::Value;
use serde_json::value;
use super::{Error, H256};

#[derive(Debug, PartialEq)]
pub enum Prf {
	HmacSha256,
}

impl Serialize for Prf {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> 
	where S: Serializer {
		match *self {
			Prf::HmacSha256 => serializer.serialize_str("hmac-sha256"),
		}
	}
}

impl Deserialize for Prf {
	fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
	where D: Deserializer {
		deserializer.deserialize(PrfVisitor)
	}
}

struct PrfVisitor;

impl Visitor for PrfVisitor {
	type Value = Prf;

	fn visit_str<E>(&mut self, value: &str) -> Result<Self::Value, E> where E: SerdeError {
		match value {
			"hmac-sha256" => Ok(Prf::HmacSha256),
			_ => Err(SerdeError::custom(Error::InvalidPrf)),
		}
	}

	fn visit_string<E>(&mut self, value: String) -> Result<Self::Value, E> where E: SerdeError {
		self.visit_str(value.as_ref())
	}
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Pbkdf2Params {
	pub c: u64,
	pub dklen: u64,
	pub prf: Prf,
	pub salt: H256,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ScryptParams {
	pub dklen: u64,
	pub p: u64,
	pub n: u64,
	pub r: u64,
	pub salt: H256,
}

#[derive(Debug, PartialEq)]
pub enum KdfParams {
	Pbkdf2(Pbkdf2Params),
	Scrypt(ScryptParams),
}

impl Serialize for KdfParams {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> 
	where S: Serializer {
		match *self {
			KdfParams::Pbkdf2(ref params) => params.serialize(serializer),
			KdfParams::Scrypt(ref params) => params.serialize(serializer),
		}
	}
}

impl Deserialize for KdfParams {
	fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
	where D: Deserializer {
		let v = try!(Value::deserialize(deserializer));

		Deserialize::deserialize(&mut value::Deserializer::new(v.clone())).map(KdfParams::Pbkdf2)
			.or_else(|_| Deserialize::deserialize(&mut value::Deserializer::new(v)).map(KdfParams::Scrypt))
			.map_err(|e| D::Error::custom(format!("{}", e)))
	}
}
