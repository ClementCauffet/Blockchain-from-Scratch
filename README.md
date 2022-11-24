# Blockchain from Scratch

PoW Blockchain implementation in Rust.

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

### Verification strategy

1. Actual index == stored index value
2. Correlation of the difficulty (just gonna trust difficulty atm -> dangerous for production applications)
3. Time continuously increasing
4. Actual previous block hash == stored prev_block_hash value (except genesis block)

Thanks to both Blockchain and Rust communities for information provided online :
Rust Language : https://www.rust-lang.org/
Algorithm Architecture : https://geeklaunch.net/blog/

**_!! This repository is still in progress. Do not use non-audited code for professionnal applications !!_**
