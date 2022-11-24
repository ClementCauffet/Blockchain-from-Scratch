use super::*;
pub struct Blockchain {
    pub blocks: Vec<Block>,
}

// impl Blockchain {
//     pub fn verify(&self) -> bool {
//         for (i, block) in self.blocks.iter().enumerate() {
//             if block.index != i as u32 {
//                 println!("Index mismatch {} != {}", &block.index, &i);
//                 return false;
//             }
//         }
//     }
// }
