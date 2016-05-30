extern crate rustc_serialize;
extern crate docopt;
extern crate ethkey;
extern crate ethstore;

use std::env;
use rustc_serialize::hex::ToHex;
use docopt::Docopt;
use ethstore::{EthStore, SecretStore, ParityDirectory, DiskDirectory, GethDirectory, KeyDirectory, DirectoryType};

pub const USAGE: &'static str = r#"
Ethereum key management.
  Copyright 2016 Ethcore (UK) Limited

Usage:
    ethstore create dir <dir> (random | prefix <p> <i> | brain <seed>)
    ethstore create (geth | parity) (random | prefix <p> <i> | brain <seed>) [--testnet]
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
    create             Create new account.
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
	cmd_list: bool,
	cmd_export: bool,
	cmd_sign: bool,
	cmd_dir: bool,
	cmd_parity: bool,
	cmd_geth: bool,
	arg_dir: String,
	arg_src: String,
	arg_dst: String,
	arg_address: String,
	arg_password: String,
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

	return if args.cmd_list {
		let dir_type = match args.flag_testnet {
			true => DirectoryType::Testnet,
			false => DirectoryType::Main,
		};

		let store = if args.cmd_dir {
			EthStore::open(DiskDirectory::at(args.arg_dir)).unwrap()
		} else if args.cmd_parity {
			EthStore::open(ParityDirectory::new(dir_type)).unwrap()
		} else {
			EthStore::open(GethDirectory::new(dir_type)).unwrap()
		};

		let result = store.accounts().into_iter()
			.enumerate()
			.map(|(i, a)| format!("#{}: {}", i, a.to_hex()))
			.collect::<Vec<String>>()
			.join("\n");
		Ok(result)
	} else if args.cmd_export {
		unimplemented!();
	} else {
		unimplemented!();
	}
}

