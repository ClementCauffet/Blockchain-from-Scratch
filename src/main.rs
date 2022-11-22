use blockchainlib::*;

fn main() {
    let mut block = Block::new(
        0,
        now(),
        vec![0; 32],
        0,
        "Genesis block".to_owned(),
        0x0000ffffffffffffffffffffffffffff,
    );

    block.hash = block.hash();

    println!("{:?}", &block);

    block.mine();

    println!("{:?}", &block);

    //MINING STRATEGY
    // 1. Generate Nonce
    // 2. Hash bytes
    // 3. Check against difficulty
    //          1. Not enought -> back to 1
    //          2. Enought -> Move on
    // 4. Add block to chain
    // 5. Submit to peers
}
