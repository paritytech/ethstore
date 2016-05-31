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
    ethstore insert dir <dir> <secret> <password>
    ethstore insert (geth | parity) <secret> <password>
    ethstore change-pwd dir <dir> <address> <old-pwd> <new-pwd>
    ethstore change-pwd (geth | parity) <address> <old-pwd> <new-pwd> [--testnet]
    ethstore list dir <dir>
    ethstore list (geth | parity) [--testnet]
    ethstore export dir <src> <dst>
    ethstore export geth parity [--testnet]
    ethstore remove dir <dir> <address> <password>
    ethstore remove (geth | parity) <address> <password> [--testnet]
    ethstore sign dir <dir> <address> <password> <message>
    ethstore sign (geth | parity) <address> <password> <message> [--testnet]
    ethstore [-h | --help]

Options:
    -h, --help         Display this message and exit.
    --testnet          Use testnet secret store.

Commands:
    insert             Save account with password.
    change-pwd         Change password.
    list               List accounts.
    export             Export accounts src to dst.
    remove             Remove account.
    sign               Sign message.
    dir                Use keystore located in directory.
    parity             Use parity keystore.
    geth               Use geth keystore.
    random             Generate new account randomly.
    prefix             Generate new account with prefixed address.
    brain              Generate new brain-wallet account.
"#;

#[derive(Debug, RustcDecodable)]
struct Args {
	cmd_insert: bool,
	cmd_change_pwd: bool,
	cmd_list: bool,
	cmd_export: bool,
	cmd_sign: bool,
	cmd_remove: bool,
	cmd_dir: bool,
	cmd_parity: bool,
	cmd_geth: bool,
	arg_secret: String,
	arg_password: String,
	arg_dir: String,
	arg_old_pwd: String,
	arg_new_pwd: String,
	arg_src: String,
	arg_dst: String,
	arg_address: String,
	arg_message: String,
	flag_testnet: bool,
}

fn main() {
	let result = execute(env::args()).unwrap();
	println!("{}", result);
}

fn execute<S, I>(command: I) -> Result<String, ()> where I: IntoIterator<Item=S>, S: AsRef<str> {
	let args: Args = Docopt::new(USAGE)
		.and_then(|d| d.argv(command).decode())
		.unwrap_or_else(|e| e.exit());

	let dir_type = match args.flag_testnet {
		true => DirectoryType::Testnet,
		false => DirectoryType::Main,
	};

	let store = if args.cmd_dir {
		EthStore::open(DiskDirectory::create(args.arg_dir).unwrap()).unwrap()
	} else if args.cmd_parity {
		EthStore::open(ParityDirectory::create(dir_type).unwrap()).unwrap()
	} else {
		EthStore::open(GethDirectory::create(dir_type).unwrap()).unwrap()
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
	} else if args.cmd_export {
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

