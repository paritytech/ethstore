use std::path::PathBuf;
use {Address, SafeAccount, Error};
use super::{KeyDirectory, DiskDirectory, DirectoryType};

fn parity_dir_path() -> PathBuf {
	unimplemented!();
}

fn parity_keystore(t: DirectoryType) -> PathBuf {
	parity_dir_path()
}

pub struct ParityDirectory {
	dir: DiskDirectory,
}

impl ParityDirectory {
	pub fn new(t: DirectoryType) -> Self {
		ParityDirectory {
			dir: DiskDirectory::at(parity_keystore(t)),
		}
	}
}

impl KeyDirectory for ParityDirectory {
	fn load(&self) -> Result<Vec<SafeAccount>, Error> {
		self.dir.load()
	}

	fn insert(&self, account: SafeAccount) -> Result<(), Error> {
		self.dir.insert(account)
	}

	fn remove(&self, address: &Address) -> Result<(), Error> {
		self.dir.remove(address)
	}
}
