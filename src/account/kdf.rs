use json;

#[derive(Debug, PartialEq)]
pub enum Prf {
	HmacSha256,
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

impl From<json::Prf> for Prf {
	fn from(json: json::Prf) -> Self {
		match json {
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

impl From<json::Pbkdf2> for Pbkdf2 {
	fn from(json: json::Pbkdf2) -> Self {
		Pbkdf2 {
			c: json.c,
			dklen: json.dklen,
			prf: From::from(json.prf),
			salt: json.salt.into(),
		}
	}
}

impl Into<json::Pbkdf2> for Pbkdf2 {
	fn into(self) -> json::Pbkdf2 {
		json::Pbkdf2 {
			c: self.c,
			dklen: self.dklen,
			prf: self.prf.into(),
			salt: From::from(self.salt),
		}
	}
}

impl From<json::Scrypt> for Scrypt {
	fn from(json: json::Scrypt) -> Self {
		Scrypt {
			dklen: json.dklen,
			p: json.p,
			n: json.n,
			r: json.r,
			salt: json.salt.into(),
		}
	}
}

impl Into<json::Scrypt> for Scrypt {
	fn into(self) -> json::Scrypt {
		json::Scrypt {
			dklen: self.dklen,
			p: self.p,
			n: self.n,
			r: self.r,
			salt: From::from(self.salt),
		}
	}
}

impl From<json::Kdf> for Kdf {
	fn from(json: json::Kdf) -> Self {
		match json {
			json::Kdf::Pbkdf2(params) => Kdf::Pbkdf2(From::from(params)),
			json::Kdf::Scrypt(params) => Kdf::Scrypt(From::from(params)),
		}
	}
}

impl Into<json::Kdf> for Kdf {
	fn into(self) -> json::Kdf {
		match self {
			Kdf::Pbkdf2(params) => json::Kdf::Pbkdf2(params.into()),
			Kdf::Scrypt(params) => json::Kdf::Scrypt(params.into()),
		}
	}
}
