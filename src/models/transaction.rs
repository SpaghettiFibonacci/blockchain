use sha2::{Digest, Sha256};

use super::wallet::Wallet;
#[derive(Clone)]
pub struct Transaction {
    pub sender: Wallet,
    pub recipient: Wallet,
    pub amount: f32,
    pub signature: String,
}

impl Transaction {
    pub fn new(sender: &Wallet, recipient: &Wallet, amount: f32) -> Self {
        Transaction {
            sender: sender.clone(),
            recipient: recipient.clone(),
            amount,
            signature: String::default(),
        }
    }

    pub fn sign(&mut self) {
        let mut hasher = Sha256::new();
        hasher.update(format!(
            "{}{}{}",
            self.sender.public_key, self.recipient.public_key, self.amount
        ));
        let hash = hasher.finalize();
        self.signature = hash_to_binary_representation(&hash);
    }

    pub fn is_valid(&self) -> bool {
        let mut hasher = Sha256::new();
        hasher.update(format!(
            "{}{}{}",
            self.sender.public_key, self.recipient.public_key, self.amount
        ));
        let hash = hasher.finalize();
        hash_to_binary_representation(&hash) == self.signature
    }
}

impl std::fmt::Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Transaction: {{\n\tsender: {},\n\trecipient: {},\n\tamount: {},\n\tsignature: {}\n}}",
            self.sender.public_key, self.recipient.public_key, self.amount, self.signature
        )
    }
}

impl std::fmt::Debug for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Transaction: {{\n\tsender: {},\n\trecipient: {},\n\tamount: {},\n\tsignature: {}\n}}",
            self.sender.public_key, self.recipient.public_key, self.amount, self.signature
        )
    }
}

fn hash_to_binary_representation(hash: &[u8]) -> String {
    let mut res: String = String::default();
    for c in hash {
        res.push_str(&format!("{:b}", c));
    }
    res
}
