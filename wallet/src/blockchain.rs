use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: f64,
    pub signature: Vec<u8>,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Block {
    pub transactions: Vec<Transaction>,
    pub hash: String,
    pub prev_hash: String,
}

#[derive(Debug)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain { blocks: vec![] }
    }

    pub fn add_transaction(&mut self, tx: Transaction) {
        let prev_hash = self.blocks.last().map_or("0".to_string(), |b| b.hash.clone());
        let block = Block {
            transactions: vec![tx.clone()],
            hash: Self::calculate_hash(&prev_hash, &[tx.clone()]),
            prev_hash,
        };
        self.blocks.push(block);
    }

    fn calculate_hash(prev_hash: &str, txs: &[Transaction]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(prev_hash);
        for tx in txs {
            hasher.update(serde_json::to_string(tx).unwrap());
        }
        format!("{:x}", hasher.finalize())
    }
}