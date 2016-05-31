use std::collections::BTreeMap;
use std::sync::RwLock;
use ethkey::Generator;
use crypto::KEY_ITERATIONS;
use {Error, Signature, SecretStore, KeyDirectory, SafeAccount, Address, Message};

pub struct EthStore {
	dir: Box<KeyDirectory>,
	iterations: u32,
	cache: RwLock<BTreeMap<Address, SafeAccount>>,
}

impl EthStore {
	pub fn open<K>(directory: K) -> Result<Self, Error> where K: KeyDirectory + 'static {
		Self::open_with_iterations(directory, KEY_ITERATIONS as u32)
	}

	pub fn open_with_iterations<K>(directory: K, iterations: u32) -> Result<Self, Error> where K: KeyDirectory + 'static {
		let accounts = try!(directory.load());
		let cache = accounts.into_iter().map(|account| (account.address.clone(), account)).collect();
		let store = EthStore {
			dir: Box::new(directory),
			iterations: iterations,
			cache: RwLock::new(cache),
		};
		Ok(store)
	}
}

impl SecretStore for EthStore {
	fn create_account<T>(&self, generator: T, password: &str) -> Result<Address, Error> where T: Generator {
		let keypair = try!(generator.generate().map_err(|_| Error::CreationFailed));

		// TODO: id?
		let id = [0u8; 16];
		let account = SafeAccount::create(&keypair, id, password, self.iterations);
		let address = keypair.address();
		{
			let mut cache = self.cache.write().unwrap();
			cache.insert(address.clone(), account);
		}

		// TODO: save to file
		Ok(address)
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

	fn sign(&self, account: &Address, password: &str, message: &Message) -> Result<Signature, Error> {
		let cache = self.cache.read().unwrap();
		let account = try!(cache.get(account).ok_or(Error::InvalidAccount));
		account.sign(password, message)
	}
}
