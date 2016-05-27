use rustc_serialize::hex::ToHex;
use ethkey::Address;
use {SafeAccount, Error};

pub trait Export {
	fn export_account(&self, address: &Address) -> Result<SafeAccount, Error>;
}

