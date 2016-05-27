use serde::{Serialize, Serializer, Deserialize, Deserializer, Error as SerdeError};
use super::{Error, H128};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Aes128CtrParams {
	pub iv: H128,
}

#[derive(Debug, PartialEq)]
pub enum Cipherparams {
	Aes128Ctr(Aes128CtrParams),
}

impl Serialize for Cipherparams {
	fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> 
	where S: Serializer {
		match *self {
			Cipherparams::Aes128Ctr(ref params) => params.serialize(serializer),
		}
	}
}

impl Deserialize for Cipherparams {
	fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
	where D: Deserializer {
		Aes128CtrParams::deserialize(deserializer)
			.map(Cipherparams::Aes128Ctr)
			.map_err(|_| Error::InvalidCipherparams)
			.map_err(SerdeError::custom)
	}
}
