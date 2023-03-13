pub mod models;
use models::block::Block;
use models::blockchain::Blockchain;
use models::transaction::Transaction;
use models::wallet::Wallet;
use sha2::Digest;
use sha2::Sha256;
use std::collections::HashMap;

use crate::models::block;

fn hash_to_binary_representation(hash: &[u8]) -> String {
    let mut res: String = String::default();
    for c in hash {
        res.push_str(&format!("{:b}", c));
    }
    res
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

    pub fn mine_block(&mut self, blockchain: &Blockchain, difficulty: u32, miner: String) -> bool {
        let mut total_balance_sender = 0f32;
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
            }

            if total_balance_sender < self.transaction.as_ref().unwrap().amount {
                return false;
            }
        }

        let mut hash = self.calculate_hash();

        while !hash.starts_with(&"0".repeat(difficulty as usize)) {
            self.nonce += 1;
            hash = self.calculate_hash();
        }
        println!("werqeqwe");

        println!("{} - {}", self.nonce, hash);
        self.hash = hash;
        true
    }
}

impl Blockchain {
    pub fn new() -> Self {
        let chain = vec![Block::new(
            0,
            String::from("0"),
            0,
            String::from("Genesis Block"),
            "".to_string(),
            None,
        )];

        Blockchain {
            chain,
            difficulty: 2,
            reward: 1f32,
            wallet_balances: HashMap::new(),
            pending_transactions: vec![],
        }
    }

    pub fn add_block(&mut self, mut block: Block) {
        let should_transact = block.mine_block(self, self.difficulty, block.miner.clone());

        self.wallet_balances.insert(
            block.miner.clone(),
            self.reward + self.wallet_balances.get(&block.miner).unwrap_or(&0f32),
        );

        if should_transact && block.transaction.is_some() {
            self.wallet_balances.insert(
                block
                    .transaction
                    .as_ref()
                    .unwrap()
                    .sender
                    .public_key
                    .clone(),
                self.wallet_balances
                    .get(
                        &block
                            .transaction
                            .as_ref()
                            .unwrap()
                            .sender
                            .public_key
                            .clone(),
                    )
                    .unwrap()
                    - block.transaction.as_ref().unwrap().amount,
            );
            self.wallet_balances.insert(
                block
                    .transaction
                    .as_ref()
                    .unwrap()
                    .recipient
                    .public_key
                    .clone(),
                self.wallet_balances
                    .get(
                        &block
                            .transaction
                            .as_ref()
                            .unwrap()
                            .recipient
                            .public_key
                            .clone(),
                    )
                    .unwrap()
                    + block.transaction.as_ref().unwrap().amount,
            );
        }
        self.chain.push(block);
    }
}

impl Default for Blockchain {
    fn default() -> Self {
        Self::new()
    }
}

fn main() {
    let mut blockchain = Blockchain::new();
    let wallet1 = Wallet::new();
    let wallet2 = Wallet::new();
    let wallet3 = Wallet::new();

    let transaction1 = Transaction::new(&wallet3, &wallet1, 2.0);
    let block = Block::new(
        1,
        blockchain.chain.last().unwrap().hash.clone(),
        0,
        String::from(""),
        wallet3.public_key.clone(),
        None,
    );
    let block2 = Block::new(
        2,
        blockchain.chain.last().unwrap().hash.clone(),
        0,
        String::from(""),
        wallet3.public_key,
        None,
    );
    let block3 = Block::new(
        3,
        blockchain.chain.last().unwrap().hash.clone(),
        0,
        String::from(""),
        wallet2.public_key,
        Some(transaction1),
    );
    blockchain.add_block(block);
    blockchain.add_block(block2);
    blockchain.add_block(block3);

    println!("{:#?}", blockchain.wallet_balances);
}
