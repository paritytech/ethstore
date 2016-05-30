use std::collections::BTreeMap;
use std::sync::RwLock;
use ethkey::{Generator, Address};
use {Error, Signature, SecretStore, KeyDirectory, SafeAccount};

pub struct EthStore {
	dir: Box<KeyDirectory>,
	cache: RwLock<BTreeMap<Address, SafeAccount>>,
}

impl EthStore {
	pub fn open<K>(directory: K) -> Result<Self, Error> where K: KeyDirectory + 'static {
		let accounts = try!(directory.load());
		let cache = accounts.into_iter().map(|account| (account.address.clone(), account)).collect();
		let store = EthStore {
			dir: Box::new(directory),
			cache: RwLock::new(cache),
		};
		Ok(store)
	}
}

impl SecretStore for EthStore {
	fn create_account<T>(&self, generator: T, password: &str) -> Address where T: Generator {
		unimplemented!();
	}

	fn accounts(&self) -> Vec<Address> {
		self.cache.read().unwrap().keys().cloned().collect()
	}

	fn change_password(&self, old_password: &str, new_password: &str) -> Result<(), Error> {
		unimplemented!();
	}

	fn remove_account(&self, account: &Address, password: &str) -> Result<(), Error> {
		unimplemented!();
	}

	fn sign(&self, account: &Address, password: &str, message: &[u8; 32]) -> Result<Signature, Error> {
		unimplemented!();
	}
}
