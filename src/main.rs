use blockchainlib::*;

fn main() {
    let difficulty = 0x000fffffffffffffffffffffffffffff;
    let mut block = Block::new(
        0,
        now(),
        vec![0; 32],
        0,
        "Genesis block".to_owned(),
        difficulty,
    );

    block.mine();
    println!("Mined Genesis Block {:?}", &block);

    let mut last_hash = block.hash.clone();

    let mut blockchain = Blockchain {
        blocks: vec![block],
    };

    for i in 1..=10 {
        let mut block = Block::new(
            i,
            now(),
            last_hash,
            0,
            "Another Block".to_owned(),
            difficulty,
        );

        block.mine();
        println!("Mined Block {:?}", &block);

        last_hash = block.hash.clone();

        blockchain.blocks.push(block);
    }
}
