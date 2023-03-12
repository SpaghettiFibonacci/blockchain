use std::collections::HashMap;

use sha2::Digest;
use sha2::Sha256;
#[derive(Debug, Clone)]
pub struct Block {
    pub id: u64,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: i64,
    pub data: String,
    pub nonce: u64,
}

#[derive(Debug, Clone)]
struct Blockchain {
    chain: Vec<Block>,
    difficulty: u32,
    reward: f32,
    wallet_balances: HashMap<String, f32>,
    // pending_transactions_vecs: Vec<Transaction>,
}

fn hash_to_binary_representation(hash: &[u8]) -> String {
    let mut res: String = String::default();
    for c in hash {
        res.push_str(&format!("{:b}", c));
    }
    res
}

impl Block {
    pub fn new(id: u64, previous_hash: String, timestamp: i64, data: String) -> Self {
        let mut block = Block {
            id,
            hash: String::default(),
            previous_hash,
            timestamp,
            data,
            nonce: 0,
        };
        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(format!(
            "{}{}{}{}{}",
            self.id, self.previous_hash, self.timestamp, self.data, self.nonce
        ));
        let hash = hasher.finalize();
        hash_to_binary_representation(&hash)
    }

    pub fn mine_block(&mut self, difficulty: u32) {
        let mut hash = self.calculate_hash();
        while !hash.starts_with(&"0".repeat(difficulty as usize)) {
            self.nonce += 1;
            hash = self.calculate_hash();
        }
        println!("{} - {}", self.nonce, hash);
        self.hash = hash;
    }
}

impl Blockchain {
    pub fn new() -> Self {
        let mut chain = vec![Block::new(
            0,
            String::from("0"),
            0,
            String::from("Genesis Block"),
        )];

        Blockchain {
            chain,
            difficulty: 2,
            reward: 1f32,
            wallet_balances: HashMap::new(),
        }
    }

    pub fn add_block(&mut self, mut block: Block) {
        block.mine_block(self.difficulty);
        self.chain.push(block);
    }
}

fn main() {
    let mut blockchain = Blockchain::new();
    blockchain.add_block(Block::new(
        1,
        blockchain.chain[0].hash.clone(),
        0,
        String::from("First Block"),
    ));
    blockchain.add_block(Block::new(
        2,
        blockchain.chain[1].hash.clone(),
        0,
        String::from("Second Block"),
    ));
    blockchain.add_block(Block::new(
        3,
        blockchain.chain[2].hash.clone(),
        0,
        String::from("Third Block"),
    ));
    println!("{:#?}", blockchain);
}
