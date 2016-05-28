use ethkey::Address;
use json;
use account::{Version, Cipher, Kdf, Pbkdf2, Scrypt, Prf};
use Error;

#[derive(Debug, PartialEq)]
pub struct Crypto {
	cipher: Cipher,
	ciphertext: [u8; 32],
	kdf: Kdf,
	mac: [u8; 32],
}

#[derive(Debug, PartialEq)]
pub struct SafeAccount {
	id: [u8; 16],
	version: Version,
	address: Address,
	crypto: Crypto,
}

impl SafeAccount {
	pub fn from_json(file: json::KeyFile) -> Result<SafeAccount, Error> {
		let result = SafeAccount {
			id: file.id.into(),
			version: Version::from(file.version),
			address: file.address.into(),
			crypto: Crypto {
				cipher: Cipher::from_json(file.crypto.cipher, file.crypto.cipherparams),
				ciphertext: file.crypto.ciphertext.into(),
				kdf: try!(Kdf::from_json(file.crypto.kdf, file.crypto.kdfparams)),
				mac: file.crypto.mac.into(),
			}
		};

		Ok(result)
	}
}

impl Into<json::KeyFile> for SafeAccount {
	fn into(self) -> json::KeyFile {
		let cipher = self.crypto.cipher.into_json();
		let kdf = self.crypto.kdf.into_json();
		json::KeyFile {
			id: From::from(self.id),
			version: self.version.into(),
			address: From::from(self.address),
			crypto: json::Crypto {
				cipher: cipher.0,
				cipherparams: cipher.1,
				ciphertext: From::from(self.crypto.ciphertext),
				kdf: kdf.0,
				kdfparams: kdf.1,
				mac: From::from(self.crypto.mac),
			}
		}
	}
}
