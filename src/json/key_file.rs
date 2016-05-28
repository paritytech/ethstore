use std::io::{Read, Write};
use serde_json;
use super::{UUID, Version, Crypto, H160};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyFile {
	pub id: UUID,
	pub version: Version,
	pub crypto: Crypto,
	pub address: H160,
}

impl KeyFile {
	pub fn load<R>(reader: R) -> Result<Self, serde_json::Error> where R: Read {
		serde_json::from_reader(reader)
	}

	pub fn write<W>(&self, writer: &mut W) -> Result<(), serde_json::Error> where W: Write {
		serde_json::to_writer(writer, self)
	}
}

#[cfg(test)]
mod tests {
	use std::str::FromStr;
	use serde_json;
	use json::{KeyFile, UUID, Version, Crypto, Cipher, Cipherparams, Aes128CtrParams, Kdf, ScryptParams, KdfParams, Prf, H128, H160, H256};

	#[test]
	fn basic_keyfile() {
		let json = r#"
		{
			"address": "6edddfc6349aff20bc6467ccf276c5b52487f7a8",
			"crypto": { 
				"cipher": "aes-128-ctr",
				"ciphertext": "7203da0676d141b138cd7f8e1a4365f59cc1aa6978dc5443f364ca943d7cb4bc",
				"cipherparams": { 
					"iv": "b5a7ec855ec9e2c405371356855fec83"
				},
				"kdf": "scrypt", 
				"kdfparams": {
					"dklen": 32,
					"n": 262144,
					"p": 1,
					"r": 8,
					"salt": "1e8642fdf1f87172492c1412fc62f8db75d796cdfa9c53c3f2b11e44a2a1b209"
				},
				"mac": "46325c5d4e8c991ad2683d525c7854da387138b6ca45068985aa4959fa2b8c8f"
			}, 
			"id": "8777d9f6-7860-4b9b-88b7-0b57ee6b3a73",
			"version": 3
		}"#;

		let expected = KeyFile {
			id: UUID::from_str("8777d9f6-7860-4b9b-88b7-0b57ee6b3a73").unwrap(),
			version: Version::V3,
			address: H160::from_str("6edddfc6349aff20bc6467ccf276c5b52487f7a8").unwrap(),
			crypto: Crypto {
				cipher: Cipher::Aes128Ctr,
				cipherparams: Cipherparams::Aes128Ctr(Aes128CtrParams {
					iv: H128::from_str("b5a7ec855ec9e2c405371356855fec83").unwrap(),
				}),
				ciphertext: H256::from_str("7203da0676d141b138cd7f8e1a4365f59cc1aa6978dc5443f364ca943d7cb4bc").unwrap(),
				kdf: Kdf::Scrypt,
				kdfparams: KdfParams::Scrypt(ScryptParams {
					n: 262144,
					dklen: 32,
					p: 1,
					r: 8,
					salt: H256::from_str("1e8642fdf1f87172492c1412fc62f8db75d796cdfa9c53c3f2b11e44a2a1b209").unwrap(),
				}),
				mac: H256::from_str("46325c5d4e8c991ad2683d525c7854da387138b6ca45068985aa4959fa2b8c8f").unwrap(),
			},
		};
		
		let keyfile: KeyFile = serde_json::from_str(json).unwrap();
		assert_eq!(keyfile, expected);
	}
}
