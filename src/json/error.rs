use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Error {
	InvalidUUID,
	UnsupportedVersion,
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
		match *self {
			Error::InvalidUUID => write!(f, "Invalid UUID"),
			Error::UnsupportedVersion => write!(f, "Unsupported version"),
		}
	}
}

impl Into<String> for Error {
	fn into(self) -> String {
		format!("{}", self)
	}
}
