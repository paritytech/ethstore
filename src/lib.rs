#![cfg_attr(feature="nightly", feature(custom_derive, plugin))]
#![cfg_attr(feature="nightly", plugin(serde_macros))]

extern crate serde;
extern crate serde_json;
extern crate rustc_serialize;
extern crate crypto;
extern crate ethkey;

mod account;
mod key_dir;
mod json;
mod error;
mod ethstore;
mod export;
mod import;
mod secret_store;

pub use self::account::SafeAccount;
pub use self::key_dir::{KeyDirectory, DiskDirectory, GethDirectory, ParityDirectory, DirectoryType};

pub use self::error::Error;
pub use self::ethstore::EthStore;
pub use self::export::Export;
pub use self::import::Import;
pub use self::secret_store::SecretStore;

pub use ethkey::{Secret, Public, Address};

pub type Signature = [u8; 65];
