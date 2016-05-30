extern crate rustc_serialize;
extern crate docopt;
extern crate ethkey;
extern crate ethstore;

use std::env;
use docopt::Docopt;

pub const USAGE: &'static str = r#"
Ethereum key management.
  Copyright 2016 Ethcore (UK) Limited

Usage:
    ethstore list dir <dir>
    ethstore list (parity | geth)
    ethstore [-h | --help]

Options:
    -h, --help         Display this message and exit.

Commands:
    list               List accounts.
    dir                Use directory.
    parity             Use parity keystore.
    geth               Use geth keystore.
"#;

#[derive(Debug, RustcDecodable)]
struct Args {
	cmd_list: bool,
	cmd_dir: bool,
	cmd_parity: bool,
	cmd_geth: bool,
	arg_dir: String,
}

fn main() {
	execute(env::args());
}

fn execute<S, I>(command: I) -> Result<String, ()> where I: IntoIterator<Item=S>, S: AsRef<str> {
	let args: Args = Docopt::new(USAGE)
		.and_then(|d| d.argv(command).decode())
		.unwrap_or_else(|e| e.exit());

	return if args.cmd_list {
		if args.cmd_dir {
			unimplemented!();
		} else if args.cmd_parity {
			unimplemented!();
		} else if args.cmd_geth {
			unimplemented!();
		} else {
			unimplemented!();
		}
	} else {
		unimplemented!();
	}
}

