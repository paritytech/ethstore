use std::path::PathBuf;
use {Address, SafeAccount, Error};
use super::{KeyDirectory, DiskDirectory};

fn geth_dir_path() -> PathBuf {
	unimplemented!();
}

pub struct GethDirectory {
	dir: DiskDirectory,
}

impl GethDirectory {
	pub fn new() -> Self {
		GethDirectory {
			dir: DiskDirectory::at(geth_dir_path()),
		}
	}
}

impl KeyDirectory for GethDirectory {
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
