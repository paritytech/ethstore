use json;

#[derive(Debug, PartialEq)]
pub struct Aes128Ctr {
	pub iv: [u8; 16],
}

#[derive(Debug, PartialEq)]
pub enum Cipher {
	Aes128Ctr(Aes128Ctr),
}

impl Cipher {
	pub fn from_json(cipher: json::Cipher, params: json::Cipherparams) -> Self {
		match (cipher, params) {
			(json::Cipher::Aes128Ctr, json::Cipherparams::Aes128Ctr(params)) => {
				Cipher::Aes128Ctr(Aes128Ctr {
					iv: params.iv.into(),
				})
			}
		}
	}

	pub fn into_json(self) -> (json::Cipher, json::Cipherparams) {
		match self {
			Cipher::Aes128Ctr(params) => {
				let cipher = json::Cipher::Aes128Ctr;
				let params = json::Cipherparams::Aes128Ctr(json::Aes128CtrParams {
					iv:	From::from(params.iv),
				});
				(cipher, params)
			}
		}
	}
}
