use {json, Error};

#[derive(Debug, PartialEq)]
pub enum Prf {
	HmacSha256,
}

impl From<json::Prf> for Prf {
	fn from(prf: json::Prf) -> Self {
		match prf {
			json::Prf::HmacSha256 => Prf::HmacSha256,
		}
	}
}

impl Into<json::Prf> for Prf {
	fn into(self) -> json::Prf {
		match self {
			Prf::HmacSha256 => json::Prf::HmacSha256,
		}
	}
}

#[derive(Debug, PartialEq)]
pub struct Pbkdf2 {
	c: u64,
	dklen: u64,
	prf: Prf,
	salt: [u8; 32],
}

#[derive(Debug, PartialEq)]
pub struct Scrypt {
	dklen: u64,
	p: u64,
	n: u64,
	r: u64,
	salt: [u8; 32],
}

#[derive(Debug, PartialEq)]
pub enum Kdf {
	Pbkdf2(Pbkdf2),
	Scrypt(Scrypt),
}

impl Kdf {
	pub fn from_json(kdf: json::Kdf, params: json::KdfParams) -> Result<Self, Error> {
		match (kdf, params) {
			(json::Kdf::Pbkdf2, json::KdfParams::Pbkdf2(params)) => {
				let result = Kdf::Pbkdf2(Pbkdf2 {
					c: params.c,
					dklen: params.dklen,
					prf: Prf::from(params.prf),
					salt: params.salt.into(),
				});
				Ok(result)
			},
			(json::Kdf::Scrypt, json::KdfParams::Scrypt(params)) => {
				let result = Kdf::Scrypt(Scrypt {
					dklen: params.dklen,
					p: params.p,
					n: params.n,
					r: params.r,
					salt: params.salt.into(),
				});
				Ok(result)
			},
			_ => {
				unimplemented!();
			}
		}
	}

	pub fn into_json(self) -> (json::Kdf, json::KdfParams) {
		match self {
			Kdf::Pbkdf2(params) => {
				let kdf = json::Kdf::Pbkdf2;
				let par = json::KdfParams::Pbkdf2(json::Pbkdf2Params {
					c: params.c,
					dklen: params.dklen,
					prf: params.prf.into(),
					salt: From::from(params.salt),
				});
				(kdf, par)
			},
			Kdf::Scrypt(params) => {
				let scrypt = json::Kdf::Scrypt;
				let par = json::KdfParams::Scrypt(json::ScryptParams {
					dklen: params.dklen,
					p: params.p,
					n: params.n,
					r: params.r,
					salt: From::from(params.salt),
				});
				(scrypt, par)
			}
		}
	}
}

