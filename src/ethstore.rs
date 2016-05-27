use ethkey::{Generator, Address};
use {Error, Signature, SecretStore};

pub struct EthStore;

impl SecretStore for EthStore {
	fn create_account<T>(&self, generator: T, password: &str) -> Address where T: Generator {
		unimplemented!();
	}

	fn accounts(&self) -> Vec<Address> {
		unimplemented!();
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
