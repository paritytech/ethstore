use std::fs;
use std::path::PathBuf;
use ethkey::Address;
use json::KeyFile;
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
		// it's not done using one iterator cause 
		// there is an issue with rustc and it takes tooo much time to compile

		// enumerate all entries in keystore
		let paths: Vec<_> = try!(fs::read_dir(&self.path))
			.flat_map(Result::ok)
			.filter(|entry| {
				let metadata = entry.metadata();
				metadata.is_ok() && !metadata.unwrap().is_dir()
			})
			.map(|entry| entry.path())
			.collect();

		// load them
		let files: Vec<_> = paths.into_iter()
			.map(fs::File::open)
			.filter_map(Result::ok)
			.collect();
			
		//transform them into safe account
		let accounts = files.into_iter()
			.map(KeyFile::load)
			.filter_map(Result::ok)
			.map(SafeAccount::from_json)
			.filter_map(Result::ok)
			.collect();
			
		Ok(accounts)
	}

	fn insert(&self, account: SafeAccount) -> Result<(), Error> {
		let id = "id";
		// transform account into key file
		let keyfile: KeyFile = account.into();
		// open the keystore directory
		let mut keyfile_path = self.path.clone();
		keyfile_path.push(id);

		// save the file
		let mut file = try!(fs::File::create(keyfile_path));
		keyfile.write(&mut file);

		Ok(())
	}

	fn remove(&self, address: &Address) -> Result<(), Error> {
		// enumerate all entries in keystore
		// find entry with given address
		// remove it
		unimplemented!();
	}
}
