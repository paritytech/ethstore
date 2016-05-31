use std::fs;
use std::path::{PathBuf, Path};
use std::collections::HashMap;
use {json, SafeAccount, Error, Address};
use super::KeyDirectory;

pub struct DiskDirectory {
	path: PathBuf,
}

impl DiskDirectory {
	pub fn at<P>(path: P) -> Self where P: AsRef<Path> {
		DiskDirectory {
			path: path.as_ref().to_path_buf(),
		}
	}

	/// all accounts found in keys directory
	fn files(&self) -> Result<HashMap<PathBuf, SafeAccount>, Error> {
		// it's not done using one iterator cause
		// there is an issue with rustc and it takes tooo much time to compile
		let paths = try!(fs::read_dir(&self.path))
			.flat_map(Result::ok)
			.filter(|entry| {
				let metadata = entry.metadata();
				metadata.is_ok() && !metadata.unwrap().is_dir()
			})
			.map(|entry| entry.path())
			.collect::<Vec<PathBuf>>();

		let files: Result<Vec<_>, _> = paths.iter()
			.map(fs::File::open)
			.collect();

		let files = try!(files);

		let accounts = files.into_iter()
			.map(json::KeyFile::load)
			.zip(paths.into_iter())
			.filter_map(|(file, path)| file.ok().map(|file| (path, SafeAccount::from(file))))
			.collect();

		Ok(accounts)
	}
}

impl KeyDirectory for DiskDirectory {
	fn load(&self) -> Result<Vec<SafeAccount>, Error> {
		let accounts = try!(self.files())
			.into_iter()
			.map(|(_, account)| account)
			.collect();
		Ok(accounts)
	}

	fn insert(&self, account: SafeAccount) -> Result<(), Error> {
		let id = "id";
		// transform account into key file
		let keyfile: json::KeyFile = account.into();
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
		// and find entry with given address
		let to_remove = try!(self.files())
			.into_iter()
			.find(|&(_, ref account)| &account.address == address);

		// remove it
		match to_remove {
			None => Err(Error::InvalidAccount),
			Some((path, _)) => fs::remove_file(path).map_err(From::from)
		}
	}
}
