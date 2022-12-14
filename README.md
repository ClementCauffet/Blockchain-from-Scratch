# Blockchain from Scratch

PoW Blockchain implementation in Rust.

General idea was to fully understand technical blockchain mechanism by implementing one from scratch.

### Description :

- **main.rs** : running basic tasks of the blockchain
- **lib.rs** : basic bytes functions needed in the program
- **hashable.rs** : computing data before hashing
- **block.rs** : main implementation, blockchain characteritics
- **blockchain.rs** : blockchain implentation and security mecanism defined in BlockValidationErr
- **transactions.rs** : transactions implementation based on UTXOs (Bitcoin based)

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

### Adding blockcs Verification strategy

1. Actual index == stored index value
2. Correlation of the difficulty (just gonna trust difficulty atm -> dangerous for production applications)
3. Time continuously increasing
4. Actual previous block hash == stored prev_block_hash value (except genesis block)

### Transactions verification strategy

1.  Where did the money come from ?
2.  Is the money available ?
3.  Who owns the money and who is sending it ?
4.  (WIP) more to cover later on cf -> https://en.bitcoin.it/wiki/Protocol_rules#.22tx.22_messages

### Transactions definition

Transactions are composed of inputs and outputs.
As a matter of fact, outputs are inputs from other transactions (UTXO-like).

Thanks to both Blockchain and Rust communities for information provided online :

Rust Language : https://www.rust-lang.org/

Algorithm Architecture : https://geeklaunch.net/blog/

**_!! This repository is still in progress. Do not use non-audited code for professionnal applications !!_**
