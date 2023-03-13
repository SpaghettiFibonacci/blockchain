use super::transaction::Transaction;

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
