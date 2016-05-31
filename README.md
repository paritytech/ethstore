# ethstore

[![Build Status][travis-image]][travis-url]

[travis-image]: https://travis-ci.org/debris/ethstore.svg?branch=master
[travis-url]: https://travis-ci.org/debris/ethstore

Ethereum key management.

[Documentation](http://debris.github.io/ethstore/ethstore/index.html)

### Usage

```
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
```

