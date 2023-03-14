use std::collections::HashMap;

use super::{block::Block, transaction::Transaction};

#[derive(Debug, Clone)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: u32,
    pub reward: f32,
    pub wallet_balances: HashMap<String, f32>,
    pub pending_transactions: Vec<Transaction>,
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
        let should_transact = block.mine_block(self, self.difficulty);

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
                    .unwrap_or(&0f32)
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
