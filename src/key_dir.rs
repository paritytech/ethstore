use std::path::PathBuf;
use ethkey::Address;
use {SafeAccount, Error};

pub trait KeyDirectory {
	fn load(&self) -> Result<Vec<SafeAccount>, Error>;
	fn insert(&self, account: SafeAccount) -> Result<(), Error>;
	fn remove(&self, address: &Address) -> Result<(), Error>;
}

pub struct DiskDirectory {
	path: PathBuf,
}

impl DiskDirectory {
	pub fn at(path: PathBuf) -> Self {
		DiskDirectory {
			path: path
		}
	}
}

impl KeyDirectory for DiskDirectory {
	fn load(&self) -> Result<Vec<SafeAccount>, Error> {
		// enumerate all entries in keystore
		// load them as key file
		// transform them into safe account
		unimplemented!();
	}

	fn insert(&self, account: SafeAccount) -> Result<(), Error> {
		// transform account into key file
		// open the keystore directory
		// save the file
		unimplemented!();
	}

	fn remove(&self, address: &Address) -> Result<(), Error> {
		// enumerate all entries in keystore
		// find entry with given address
		// remove it
		unimplemented!();
	}
}
