#![cfg_attr(feature="nightly", feature(custom_derive, plugin))]
#![cfg_attr(feature="nightly", plugin(serde_macros))]

extern crate libc;
extern crate rand;
extern crate serde;
extern crate serde_json;
extern crate rustc_serialize;
extern crate crypto as rcrypto;
extern crate tiny_keccak;
extern crate ethkey;

mod account;
mod json;
mod key_dir;
mod crypto;

mod error;
mod ethstore;
mod import;
mod primitives;
mod random;
mod secret_store;

pub use self::account::SafeAccount;
pub use self::key_dir::{KeyDirectory, DiskDirectory, GethDirectory, ParityDirectory, DirectoryType};

pub use self::error::Error;
pub use self::ethstore::EthStore;
pub use self::import::import_accounts;
pub use self::primitives::{Secret, Public, Address, Message, Signature};
pub use self::secret_store::SecretStore;

