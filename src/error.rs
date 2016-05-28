use std::io::Error as IoError;

#[derive(Debug)]
pub enum Error {
	Io(IoError),
}

impl From<IoError> for Error {
	fn from(err: IoError) -> Self {
		Error::Io(err)
	}
}
