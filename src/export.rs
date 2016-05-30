use {SafeAccount, Error, Address};

pub trait Export {
	fn export_account(&self, address: &Address) -> Result<SafeAccount, Error>;
}

