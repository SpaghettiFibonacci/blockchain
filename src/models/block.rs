use sha2::{Digest, Sha256};

use super::{blockchain::Blockchain, transaction::Transaction};

#[derive(Debug, Clone)]
pub struct Block {
    pub id: u64,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: i64,
    pub data: String,
    pub transaction: Option<Transaction>,
    pub miner: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(
        id: u64,
        previous_hash: String,
        timestamp: i64,
        data: String,
        miner: String,
        transaction: Option<Transaction>,
    ) -> Self {
        let mut block = Block {
            id,
            hash: String::default(),
            previous_hash,
            timestamp,
            data,
            nonce: 0,
            miner,
            transaction,
        };
        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(format!(
            "{}{}{}{}{:?}{:?}{}",
            self.id,
            self.previous_hash,
            self.timestamp,
            self.data,
            self.miner,
            self.transaction,
            self.nonce
        ));
        let hash = hasher.finalize();
        hash_to_binary_representation(&hash)
    }

    pub fn mine_block(&mut self, blockchain: &Blockchain, difficulty: u32) -> bool {
        let mut total_balance_sender = 0f32;
        let mut return_value = true;
        if self.transaction.is_some() {
            for block in &blockchain.chain {
                if block.transaction.is_some()
                    && self.transaction.as_ref().unwrap().sender.public_key
                        == block.transaction.as_ref().unwrap().sender.public_key
                    && block.transaction.as_ref().unwrap().sender.public_key
                        == self.transaction.as_ref().unwrap().sender.public_key
                {
                    total_balance_sender += block.transaction.as_ref().unwrap().amount;
                }

                if block.miner == self.transaction.as_ref().unwrap().sender.public_key {
                    total_balance_sender += blockchain.reward;
                }
            }

            if total_balance_sender < self.transaction.as_ref().unwrap().amount {
                return_value = false;
            }
        }

        let mut hash = self.calculate_hash();

        while !hash.starts_with(&"0".repeat(difficulty as usize)) {
            self.nonce += 1;
            hash = self.calculate_hash();
        }

        println!("{} - {}", self.nonce, hash);
        self.hash = hash;
        return_value
    }
}

fn hash_to_binary_representation(hash: &[u8]) -> String {
    let mut res: String = String::default();
    for c in hash {
        res.push_str(&format!("{:b}", c));
    }
    res
}
