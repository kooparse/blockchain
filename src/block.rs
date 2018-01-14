use helpers::{hash_from_string, prepare_string_to_hash, Index};

#[derive(Clone, Debug)]
pub struct Block {
    pub index: Index,
    pub hash: String,
    previous_hash: String,
    pub data: String,
}

impl Block {
    pub fn new(idx: Index, hash: &str, prev: &str, data: &str) -> Block {
        Block {
            index: idx,
            hash: String::from(hash),
            previous_hash: String::from(prev),
            data: String::from(data),
        }
    }

    pub fn generate_genesis() -> Block {
        const INDEX: Index = 0;
        let data: String = String::from("That's one small step for a man");
        let previous_hash: String = hash_from_string(data.clone());
        let hash: String = prepare_string_to_hash(INDEX, &previous_hash, &data);
        Block::new(INDEX, &hash, &previous_hash, &data)
    }
}
