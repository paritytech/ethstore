use std::io::Error as IoError;
use ethkey::Error as EthKeyError;

#[derive(Debug)]
pub enum Error {
	Io(IoError),
	InvalidPassword,
	InvalidSecret,
	InvalidAccount,
	CreationFailed,
	EthKey(EthKeyError),
}

impl From<IoError> for Error {
	fn from(err: IoError) -> Self {
		Error::Io(err)
	}
}

impl From<EthKeyError> for Error {
	fn from(err: EthKeyError) -> Self {
		Error::EthKey(err)
	}
}
