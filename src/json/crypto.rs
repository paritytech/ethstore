use super::{Cipher, Cipherparams, Kdf, KdfParams, H256};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Crypto {
	pub cipher: Cipher,
	pub cipherparams: Cipherparams,
	pub ciphertext: H256,
	pub kdf: Kdf,
	pub kdfparams: KdfParams,
	pub mac: H256,
}

