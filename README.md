# Blockchain from Scratch

PoW Blockchain implentation in Rust.

General idea was to fully understand technical blockchain mechanism by implementing one from scratch.

### Description :

- **main.rs** : running basic tasks of the blockchain
- **lib.rs** : basic bytes functions needed in the program
- **hashable.rs** : computing data before hashing
- **block.rs** : main implementation, blockchain characteritics

### Installation

Install rust : https://www.rust-lang.org/

```
git clone https://github.com/ClementCauffet/Blockchain-from-Scratch.git
cd blockchain-from-scratch
cargo run
```

### Mining strategy

1.  Generate Nonce
2.  Hash bytes (SHA-256)
3.  Check against difficulty
    - Not enought -> back to 1.
    - Enought -> Move on to 4.
4.  Add block to chain
5.  Submit to peers

Thanks to both Blockchain and Rust communities for information provided online.

**_!! This repository is still in progress. Do not use non-audited code for professionnal applications !!_**
