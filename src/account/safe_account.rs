use {json, Address};
use account::{Version, Cipher, Kdf};

#[derive(Debug, PartialEq)]
pub struct Crypto {
	pub cipher: Cipher,
	pub ciphertext: [u8; 32],
	pub kdf: Kdf,
	pub mac: [u8; 32],
}

#[derive(Debug, PartialEq)]
pub struct SafeAccount {
	pub id: [u8; 16],
	pub version: Version,
	pub address: Address,
	pub crypto: Crypto,
}

impl From<json::Crypto> for Crypto {
	fn from(json: json::Crypto) -> Self {
		Crypto {
			cipher: From::from(json.cipher),
			ciphertext: json.ciphertext.into(),
			kdf: From::from(json.kdf),
			mac: json.mac.into(),
		}
	}
}

impl Into<json::Crypto> for Crypto {
	fn into(self) -> json::Crypto {
		json::Crypto {
			cipher: self.cipher.into(),
			ciphertext: From::from(self.ciphertext),
			kdf: self.kdf.into(),
			mac: From::from(self.mac),
		}
	}
}

impl From<json::KeyFile> for SafeAccount {
	fn from(json: json::KeyFile) -> Self {
		SafeAccount {
			id: json.id.into(),
			version: From::from(json.version),
			address: json.address.into(),
			crypto: From::from(json.crypto),
		}
	}
}

impl Into<json::KeyFile> for SafeAccount {
	fn into(self) -> json::KeyFile {
		json::KeyFile {
			id: From::from(self.id),
			version: self.version.into(),
			address: From::from(self.address),
			crypto: self.crypto.into(),
		}
	}
}
