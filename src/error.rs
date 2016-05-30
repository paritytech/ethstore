use std::io::Error as IoError;

#[derive(Debug)]
pub enum Error {
	Io(IoError),
	InvalidPassword,
}

impl From<IoError> for Error {
	fn from(err: IoError) -> Self {
		Error::Io(err)
	}
}
