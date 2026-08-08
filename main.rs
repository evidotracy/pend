use sha2::{Sha256, Digest};
use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, Verifier};
use hex;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug)]
struct Transaction {
    from: String,
    to: String,
    amount: u64,
    timestamp: u64,
}

#[derive(Clone, Debug)]
struct Block {
    index: u64,
    timestamp: u64,
    transactions: Vec<Transaction>,
    previous_hash: String,
    hash: String,
}

struct Blockchain {
    chain: Vec<Block>,
    pending_transactions: Vec<Transaction>,
    balances: HashMap<String, u64>,
}

impl Blockchain {
    fn new() -> Self {
        Blockchain {
            chain: vec![],
            pending_transactions: vec![],
            balances: HashMap::new(),
        }
    }

    fn create_genesis_block(&mut self) {
        let genesis_block = Block {
            index: 0,
            timestamp: Self::get_timestamp(),
            transactions: vec![],
            previous_hash: "0".to_string(),
            hash: calculate_hash("genesis")
        };
        self.chain.push(genesis_block);
    }

    fn get_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }

    fn add_transaction(&mut self, from: String, to: String, amount: u64) {
        let transaction = Transaction {
            from,
            to,
            amount,
            timestamp: Self::get_timestamp(),
        };
        self.pending_transactions.push(transaction);
    }

    fn mine_block(&mut self, miner: String) {
        let previous_hash = self.chain.last().unwrap().hash.clone();
        let mut block = Block {
            index: self.chain.len() as u64,
            timestamp: Self::get_timestamp(),
            transactions: self.pending_transactions.clone(),
            previous_hash,
            hash: String::new(),
        };
        block.hash = calculate_hash(&format!("{}{}{:?}", block.index, block.previous_hash, block.transactions));
        self.chain.push(block);
        self.pending_transactions.clear();
        
        *self.balances.entry(miner).or_insert(0) += 10;
    }
}
fn main() {
let mut blockchain = Blockchain::new();
    blockchain.create_genesis_block();
    
    blockchain.add_transaction("Alice".to_string(), "Bob".to_string(), 50);
    blockchain.add_transaction("Bob".to_string(), "Charlie".to_string(), 30);
    
    blockchain.mine_block("Miner1".to_string());
    
    println!("Blockchain created!");
    println!("Total blocks: {}", blockchain.chain.len());
    println!("Pending transactions: {}", blockchain.pending_transactions.len());
  }
fn calculate_hash(data: &str) -> String {
let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}
