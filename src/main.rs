use blockchainlib::*;

fn main() {
    //Used later on to print transactions
    fn print_transactions(block: &Block) {
        for transaction in &block.transactions {
            println!("Transaction:");
            println!("  Inputs:");
            for input in &transaction.inputs {
                println!("    {:?}", input);
            }
            println!("  Outputs:");
            for output in &transaction.outputs {
                println!("    To address: {}", output.to_addr);
                println!("    Value: {}", output.value);
            }
        }
    }

    // Hardcoded Difficulty -> education purpose / should depend on the hash power of the entire network (to get approx. X minutes / block)
    let difficulty = 0x000fffffffffffffffffffffffffffff;

    //Genesis Block -> no backtracing data as it's the first block
    let mut genesis_block = Block::new(
        0,
        now(),
        vec![0; 32],
        vec![Transaction {
            inputs: vec![],
            outputs: vec![
                transaction::Output {
                    to_addr: "Alice".to_owned(),
                    value: 35,
                },
                transaction::Output {
                    to_addr: "Bob".to_owned(),
                    value: 25,
                },
                transaction::Output {
                    to_addr: "Chris".to_owned(),
                    value: 15,
                },
            ],
        }],
        difficulty,
    );

    genesis_block.mine();

    println!(
        "
    ===== NEW BLOCK ====="
    );
    println!("Mined genesis block {:?}", &genesis_block);

    // Print detail
    print_transactions(&genesis_block);
    println!(
        "=====================
    "
    );

    let mut last_hash = genesis_block.hash.clone();

    let mut blockchain = Blockchain::new();

    blockchain
        .update_with_block(genesis_block)
        .expect("Failed to add genesis block");

    let mut block = Block::new(
        1,
        now(),
        last_hash,
        vec![
            Transaction {
                inputs: vec![],
                outputs: vec![transaction::Output {
                    to_addr: "Chris".to_owned(),
                    value: 536,
                }],
            },
            Transaction {
                inputs: vec![blockchain.blocks[0].transactions[0].outputs[0].clone()],
                outputs: vec![
                    transaction::Output {
                        to_addr: "Alice".to_owned(),
                        value: 360,
                    },
                    transaction::Output {
                        to_addr: "Bob".to_owned(),
                        value: 12,
                    },
                ],
            },
        ],
        difficulty,
    );

    block.mine();

    println!("===== NEW BLOCK =====");
    println!("Mined block {:?}", &block);
    // Print detail
    print_transactions(&block);
    println!(
        "=====================

    "
    );

    last_hash = block.hash.clone();

    blockchain
        .update_with_block(block)
        .expect("Failed to add block");

    let mut block2 = Block::new(
        2,
        now(),
        last_hash,
        vec![
            Transaction {
                inputs: vec![],
                outputs: vec![transaction::Output {
                    to_addr: "Chris".to_owned(),
                    value: 536,
                }],
            },
            Transaction {
                inputs: vec![blockchain.blocks[1].transactions[1].outputs[1].clone()],
                outputs: vec![transaction::Output {
                    to_addr: "Daniel".to_owned(),
                    value: 25,
                }],
            },
        ],
        difficulty,
    );

    block2.mine();

    println!("===== NEW BLOCK =====");
    println!("Mined block {:?}", &block2);

    // Print detail
    print_transactions(&block2);
    println!(
        "=====================

    "
    );

    blockchain
        .update_with_block(block2)
        .expect("Failed to add block");
}
