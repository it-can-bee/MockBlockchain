use crate::block;

pub struct BlockChain {
    pub blocks: Vec<block::Block>,
    // pub curr_bits: u32,
}

// const INIT_BITS: u32 = 0x1d00ffff; //Actually should use this value
// const INIT_BITS: u32 = 0x1d00ffff;

impl BlockChain {
    //add blocks
    pub fn add_block(&mut self, data: String) {
        let prev_block = &self.blocks[self.blocks.len() - 1];
        // let new_block = block::Block::new_block(data, prev_block.hash.clone(), self.curr_bits);
        let new_block = block::Block::new_block(data, prev_block.hash.clone());
        self.blocks.push(new_block);
    }
    //create genesis block
    fn new_genesis_block() -> block::Block {
        block::Block::new_block("Genesis Block".to_string(), "".to_string())
    }
    //create blockchain
    pub fn new_blockchain() -> BlockChain {
        BlockChain {
            blocks: vec![BlockChain::new_genesis_block()],
            // curr_bits: INIT_BITS,
        }
    }

}