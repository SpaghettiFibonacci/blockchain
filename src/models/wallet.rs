use std::time::SystemTime;

use sha2::{Digest, Sha256};
#[derive(Clone)]
pub struct Wallet {
    pub private_key: String,
    pub public_key: String,
}

impl Wallet {
    pub fn new() -> Self {
        let mut hasher = Sha256::new();
        hasher.update(format!(
            "{}",
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        ));
        let private_key = hash_to_binary_representation(&hasher.finalize());
        let mut hasher = Sha256::new();
        hasher.update(private_key.clone());
        let public_key = hash_to_binary_representation(&hasher.finalize());
        Wallet {
            private_key,
            public_key,
        }
    }
}

impl Default for Wallet {
    fn default() -> Self {
        Self::new()
    }
}

fn hash_to_binary_representation(hash: &[u8]) -> String {
    let mut res: String = String::default();
    for c in hash {
        res.push_str(&format!("{:b}", c));
    }
    res
}
