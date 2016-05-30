use {json, Address, Secret, Error, crypto};
use crypto::Keccak256;
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

impl SafeAccount {
	pub fn secret(&self, password: &str) -> Result<Secret, Error> {
		let (derived_left_bits, derived_right_bits) = match self.crypto.kdf {
			Kdf::Pbkdf2(ref params) => crypto::derive_key_iterations(password, &params.salt, params.c),
			Kdf::Scrypt(ref params) => crypto::derive_key_scrypt(password, &params.salt, params.n, params.p, params.r),
		};

		let mac = crypto::derive_mac(&derived_right_bits, &self.crypto.ciphertext).keccak256();

		if mac != self.crypto.mac {
			return Err(Error::InvalidPassword);
		}

		let mut secret = [0u8; 32];

		match self.crypto.cipher {
			Cipher::Aes128Ctr(ref params) => {
				crypto::aes::decrypt(&derived_left_bits, &params.iv, &self.crypto.ciphertext, &mut secret)
			},
		}

		Ok(secret)
	}
}
