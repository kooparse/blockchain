use block::Block;
use helpers::hash_from_string;

pub struct Blockchain {
    blockchain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        Blockchain {
            blockchain: vec![Block::generate_genesis()],
        }
    }

    pub fn generate_block(&mut self, data: &str) {
        let previous_block: Block = self.get_last_block();

        let index: i32 = previous_block.index + 1;
        let hash = hash_from_string(format!(
            "{}{}{}",
            index, previous_block.hash, data
        ));
        let new_block = Block::new(index, &hash, &previous_block.hash, &data);
        self.add_block(new_block);
    }

    pub fn print_last_block(&self) {
        println!("{:?}", self.get_last_block());
    }

    fn add_block(&mut self, block: Block) {
        self.blockchain.push(block);
    }

    fn get_last_block(&self) -> Block {
        let length = self.blockchain.len();
        self.blockchain[length - 1].clone()
    }
}
