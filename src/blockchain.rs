use super::*;
pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn verify(&self) -> bool {
        for (i, block) in self.blocks.iter().enumerate() {
            if block.index != i as u32 {
                println!("Index mismatch {} != {}", &block.index, &i);
                return false;
            } else if !block::check_difficulty(&block.hash(), block.difficulty) {
                println!("Difficulty mismatch");
                return false;
            } else if i != 0 {
                //Not genesis Block
                let prev_block = &self.blocks[i - 1];
                if block.timestamp <= prev_block.timestamp {
                    println!("Time mismatch");
                    return false;
                } else if block.prev_block_hash != prev_block.hash {
                    println!("Hash mismatch");
                    return false;
                }
            } else {
                //Genesis Block
                if block.prev_block_hash != vec![0; 32] {
                    println!("Genesis block prev_block_hash mismatch");
                    return false;
                }
            }
        }
        return true;
    }
}
