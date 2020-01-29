# bitcoin-address-tutorial

Rust CLI application that generates different Bitcoin addresses 

- [x] P2PKH
- [x] P2SH
- [ ] P2WSH ( WIP )

## Help

1. Generate P2PKH address

```sh
$ cargo run -- --type p2sh --private_key a966eb6058f8ec9f47074a2faadd3dab42e2c60ed05bc34d39d6c0e1d32b8bdf
```
Output

```
================= Generating P2PKH Address =================
P2PKH Address: 
16JrGhLx5bcBSA34kew9V6Mufa4aXhFe9X
======================================
```

2. Generate simple P2SH address

```sh
$ cargo run -- --type p2pkh --spending_pub_key 020ae29f86f404e4b302cfa17ff15d93149af6a54c80a4172d47e41f55f6a78d73
```

Output

```sh
================= Generating P2SH address =================
Redeem Script: 
21020ae29f86f404e4b302cfa17ff15d93149af6a54c80a4172d47e41f55f6a78d73ac
P2SH Address: 
38RgUAR367PFbFFgS57BYcERHkpqHEMBvA
======================================
```

3. Generate 2-3 Multisig P2SH address

```sh
cargo run -- --type p2sh --spending_pub_key 020ae29f86f404e4b302cfa17ff15d93149af6a54c80a4172d47e41f55f6a78d73,03664d528eb80096671ef9011c533ceb5df133238e3690d88f2960c786398b86b1,029a449ea4a2155ea10002d704604bb3e8606631d35af20889a74b82b2dab572f6
```
Output

```sh
================= Generating P2SH address =================
Redeem Script: 
5221020ae29f86f404e4b302cfa17ff15d93149af6a54c80a4172d47e41f55f6a78d732103664d528eb80096671ef9011c533ceb5df133238e3690d88f2960c786398b86b121029a449ea4a2155ea10002d704604bb3e8606631d35af20889a74b82b2dab572f653ae
P2SH Address: 
3DD4YP2T75TQtf84KrHzYVLYgNAeaHWqxq
======================================
```

4. Generate sample public and compressed private key pairs

```sh
$ cargo run -- -k                       
```

Output

```sh
================= key pair =================
Private Key
3b76757db8d6cef7ebb95c96f2ab67f947d09b76692cefcb3b916aead1631dc6
Public Key
02b026bbc7233a3f5a0c107b5f74578712a7db9bf8
======================================
```
# LICENSE
MIT