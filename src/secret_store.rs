use ethkey::{Generator};
use {Error, Signature, Address, Message, Secret};

pub trait SecretStore {
	fn create_account<T>(&self, generator: T, password: &str) -> Result<Address, Error> where T: Generator;

	fn insert_account(&self, secret: Secret, password: &str) -> Result<Address, Error>;

	fn accounts(&self) -> Vec<Address>;

	fn change_password(&self, account: &Address, old_password: &str, new_password: &str) -> Result<(), Error>;

	fn remove_account(&self, account: &Address, password: &str) -> Result<(), Error>;

	fn sign(&self, account: &Address, password: &str, message: &Message) -> Result<Signature, Error>;
}

