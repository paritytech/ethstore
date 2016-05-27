use {SafeAccount, Error};

pub trait Import {
	fn import_account(&self, account: SafeAccount) -> Result<(), Error>;
}
