use super::{UUID, Version, Crypto};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct KeyFile {
	id: UUID,
	version: Version,
	crypto: Crypto,
}

#[cfg(test)]
mod tests {
	use std::str::FromStr;
	use serde_json;
	use json::{KeyFile, UUID, Version, Crypto, Cipher, Cipherparams, Aes128CtrParams, Kdf, Pbkdf2Params, KdfParams, Prf, H128, H256};

	#[test]
	fn basic_keyfile() {
		let json = r#"
		{
			"crypto" : {
				"cipher" : "aes-128-ctr",
				"cipherparams" : {
					"iv" : "6087dab2f9fdbbfaddc31a909735c1e6"
				},
				"ciphertext" : "5318b4d5bcd28de64ee5559e671353e16f075ecae9f99c7a79a38af5f869aa46",
				"kdf" : "pbkdf2",
				"kdfparams" : {
					"c" : 262144,
					"dklen" : 32,
					"prf" : "hmac-sha256",
					"salt" : "ae3cd4e7013836a3df6bd7241b12db061dbe2c6785853cce422d148a624ce0bd"
				},
				"mac" : "517ead924a9d0dc3124507e3393d175ce3ff7c1e96529c6c555ce9e51205e9b2"
			},
			"id" : "3198bc9c-6672-5ab3-d995-4942343ae5b6",
			"version" : 3
		}"#;

		let expected = KeyFile {
			id: UUID::from_str("3198bc9c-6672-5ab3-d995-4942343ae5b6").unwrap(),
			version: Version::V3,
			crypto: Crypto {
				cipher: Cipher::Aes128Ctr,
				cipherparams: Cipherparams::Aes128Ctr(Aes128CtrParams {
					iv: H128::from_str("6087dab2f9fdbbfaddc31a909735c1e6").unwrap(),
				}),
				ciphertext: H256::from_str("5318b4d5bcd28de64ee5559e671353e16f075ecae9f99c7a79a38af5f869aa46").unwrap(),
				kdf: Kdf::Pbkdf2,
				kdfparams: KdfParams::Pbkdf2(Pbkdf2Params {
					c: 262144,
					dklen: 32,
					prf: Prf::HmacSha256,
					salt: H256::from_str("ae3cd4e7013836a3df6bd7241b12db061dbe2c6785853cce422d148a624ce0bd").unwrap(),
				}),
				mac: H256::from_str("517ead924a9d0dc3124507e3393d175ce3ff7c1e96529c6c555ce9e51205e9b2").unwrap(),
			},
		};
		
		let keyfile: KeyFile = serde_json::from_str(json).unwrap();
		assert_eq!(keyfile, expected);
	}
}
