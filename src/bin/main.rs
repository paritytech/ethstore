extern crate rustc_serialize;
extern crate docopt;
extern crate ethkey;
extern crate ethstore;

use std::env;
use std::str::FromStr;
use docopt::Docopt;
use ethstore::{EthStore, SecretStore, ParityDirectory, DiskDirectory, GethDirectory, DirectoryType, Secret, Address, Message};

pub const USAGE: &'static str = r#"
Ethereum key management.
  Copyright 2016 Ethcore (UK) Limited

Usage:
    ethstore insert <secret> <password> [--dir DIR]
    ethstore change-pwd <address> <old-pwd> <new-pwd> [--dir DIR]
    ethstore list [--dir DIR]
    ethstore import <src> [--dir DIR]
    ethstore remove <address> <password> [--dir DIR]
    ethstore sign <address> <password> <message> [--dir DIR]
    ethstore [-h | --help]

Options:
    -h, --help         Display this message and exit.
    --dir DIR          Specify the secret store directory. It may be either
                       parity, parity-test, geth, geth-test
                       or a path [default: parity].

Commands:
    insert             Save account with password.
    change-pwd         Change password.
    list               List accounts.
    import             Import accounts from src.
    remove             Remove account.
    sign               Sign message.
"#;

#[derive(Debug, RustcDecodable)]
struct Args {
	cmd_insert: bool,
	cmd_change_pwd: bool,
	cmd_list: bool,
	cmd_import: bool,
	cmd_remove: bool,
	cmd_sign: bool,
	arg_secret: String,
	arg_password: String,
	arg_old_pwd: String,
	arg_new_pwd: String,
	arg_src: String,
	arg_address: String,
	arg_message: String,
	flag_dir: String,
}

fn main() {
	let result = execute(env::args()).unwrap();
	println!("{}", result);
}

fn execute<S, I>(command: I) -> Result<String, ()> where I: IntoIterator<Item=S>, S: AsRef<str> {
	let args: Args = Docopt::new(USAGE)
		.and_then(|d| d.argv(command).decode())
		.unwrap_or_else(|e| e.exit());

	let store = match args.flag_dir.as_ref() {
		"parity" => EthStore::open(ParityDirectory::create(DirectoryType::Main).unwrap()).unwrap(),
		"parity-test" => EthStore::open(ParityDirectory::create(DirectoryType::Testnet).unwrap()).unwrap(),
		"geth" => EthStore::open(GethDirectory::create(DirectoryType::Main).unwrap()).unwrap(),
		"geth-test" => EthStore::open(GethDirectory::create(DirectoryType::Testnet).unwrap()).unwrap(),
		path => EthStore::open(DiskDirectory::create(path).unwrap()).unwrap(),
	};

	return if args.cmd_insert {
		let secret = Secret::from_str(&args.arg_secret).unwrap();
		let address = store.insert_account(secret, &args.arg_password).unwrap();
		Ok(format!("{}", address))
	} else if args.cmd_change_pwd {
		let address = Address::from_str(&args.arg_address).unwrap();
		let ok = store.change_password(&address, &args.arg_old_pwd, &args.arg_new_pwd).is_ok();
		Ok(format!("{}", ok))
	} else if args.cmd_list {
		let result = store.accounts().into_iter()
			.map(|a| format!("{}", a))
			.collect::<Vec<String>>()
			.join("\n");
		Ok(result)
	} else if args.cmd_import {
		unimplemented!();
	} else if args.cmd_remove {
		let address = Address::from_str(&args.arg_address).unwrap();
		let ok = store.remove_account(&address, &args.arg_password).is_ok();
		Ok(format!("{}", ok))
	} else if args.cmd_sign {
		let address = Address::from_str(&args.arg_address).unwrap();
		let message = Message::from_str(&args.arg_message).unwrap();
		let signature = store.sign(&address, &args.arg_password, &message).unwrap();
		Ok(format!("{}", signature))
	} else {
		unreachable!();
	}
}

