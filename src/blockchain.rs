use block::Block;

pub struct Blockchain {
    blocks: Vec<Block>
}

impl Blockchain {
    pub fn new() -> Blockchain {
        let mut blocks = Vec::new();
        blocks.push(Blockchain::generate_genesis_block());
        Blockchain { blocks }
    }

    fn generate_genesis_block() -> Block {
        let content = String::from("Genesis block");
        let previous_hash =
            String::from("0000000000000000000000000000000000000000000000000000000000000000");
        Block::new(0, &previous_hash, &content)
    }

    pub fn blocks(&self) -> &Vec<Block> {
        &self.blocks
    }

    pub fn block(&self, index: usize) -> Option<&Block> {
        if self.blocks.len() > 0 && self.blocks.len() < index {
            Some(&self.blocks[index])
        } else {
            None
        }
    }

    pub fn last_block(&self) -> Option<&Block> {
        match self.blocks.len() {
            0 => None,
            n => Some(&self.blocks[n - 1])
        }
    }

    pub fn push(&mut self, block: Block) -> () {
        self.blocks.push(block)
    }

    pub fn len(&self) -> usize {
        self.blocks.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_blockchain_with_genesis_block() {
        let blockchain = Blockchain::new();
        let block = blockchain.block(0).unwrap();
        assert_eq!(block.content(), "Genesis block");
        assert_eq!(block.previous_hash(),
                   "0000000000000000000000000000000000000000000000000000000000000000");
    }

    #[test]
    fn get_last_block() {
        let blockchain = Blockchain::new();
        assert_eq!(blockchain.blocks().len(), 1);
        assert_eq!(blockchain.last_block().unwrap().index(), 0);
    }

    #[test]
    fn push_new_block_into_blockchain() {
        let mut blockchain = Blockchain::new();
        let last_block = blockchain.last_block().cloned().unwrap();
        let content = String::from("Next block");
        let previous_hash = last_block.hash().to_string();

        let block = Block::new(last_block.index() + 1, &previous_hash, &content);
        blockchain.push(block);

        let pushed_block = blockchain.last_block().cloned().unwrap();
        assert_eq!(blockchain.len(), 2);
        assert_eq!(pushed_block.previous_hash(), last_block.hash());
        assert_eq!(pushed_block.index(), last_block.index() + 1);
        assert_eq!(pushed_block.content(), content);
    }
}
