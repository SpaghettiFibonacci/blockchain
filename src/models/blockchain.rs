use std::collections::HashMap;

use super::{block::Block, transaction::Transaction};

#[derive(Debug, Clone)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: u32,
    pub reward: f32,
    pub wallet_balances: HashMap<String, f32>,
    // pub pending_transactions: Vec<Transaction>,
}
