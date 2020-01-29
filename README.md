# bitcoin-address-tutorial

Rust CLI application that generates different Bitcoin addresses 

- [x] P2PKH
- [x] P2SH
- [ ] P2WSH ( WIP )

## Help

1. Generate P2PKH address

```sh
$ cargo run -- --type p2pkh --private_key a966eb6058f8ec9f47074a2faadd3dab42e2c60ed05bc34d39d6c0e1d32b8bdf
```

2. Generate simple P2SH address

```sh
$ cargo run -- --type p2pkh --spending_pub_key 020ae29f86f404e4b302cfa17ff15d93149af6a54c80a4172d47e41f55f6a78d73
```

3. Generate 2-3 Multisig P2SH address

```sh
cargo run -- --type p2pkh --spending_pub_key 020ae29f86f404e4b302cfa17ff15d93149af6a54c80a4172d47e41f55f6a78d73,020ae29f86f404e4b302cfa17ff15d93149af6a54c80a4172d47e41f55f6a78d73,020ae29f86f404e4b302cfa17ff15d93149af6a54c80a4172d47e41f55f6a78d73
```

# LICENSE
MIT