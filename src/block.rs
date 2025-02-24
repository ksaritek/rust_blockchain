use num_bigint::{BigInt};
use serde::{Deserialize, Serialize};
use sled::IVec;
use std::ops::ShlAssign;
use crate::transactions::Transaction;

#[derive(Clone, Serialize, Deserialize)]
pub struct Block {
    timestamp: i64,
    pre_block_hash: String,
    hash: String,
    transactions: Vec<Transaction>,
    nonce: i64,
    height: usize,
}

impl Block{
    pub fn new_block(pre_block_hash: String, transactions: &[Transaction], height: usize) -> Block {
        let mut block = Block {
            timestamp: crate::current_timestamp(),
            pre_block_hash,
            hash: String::new(),
            transactions: transactions.to_vec(),
            nonce: 0,
            height
        };

        let pow = ProofOfWork::new_proof_of_work(block.clone());
        let (nonce, hash) = pow.run();
        block.nonce = nonce;
        block.hash = hash;

        block
    }

    pub fn deserialize(bytes: &[u8]) -> bincode::Result<Block> {
        bincode::deserialize(bytes)
    }

   
    pub fn serialize(&self) -> bincode::Result<Vec<u8>> {
        bincode::serialize(self)
    }


    pub fn generate_genesis_block(transaction: &Transaction) -> Block {
        let transactions = vec![transaction.clone()];
        return Block::new_block(String::from("None"), &transactions, 0);
    }

    pub fn hash_transactions(&self) -> Vec<u8> {
        let mut txhashs = vec![];
        for transaction in &self.transactions {
            txhashs.extend(transaction.get_id());
        }
        crate::sha256_digest(txhashs.as_slice())
    }

    pub fn get_transactions(&self) -> &[Transaction] {
        self.transactions.as_slice()
    }

    pub fn get_pre_block_hash(&self) -> String {
        self.pre_block_hash.clone()
    }

    pub fn get_hash(&self) -> &str {
        self.hash.as_str()
    }

    pub fn get_hash_bytes(&self) -> Vec<u8> {
        self.hash.as_bytes().to_vec()
    }

    pub fn get_timestamp(&self) -> i64 {
        self.timestamp
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

}


impl From<Block> for IVec {
    fn from(b: Block) -> Self {
        let bytes = bincode::serialize(&b).expect("Failed to serialize Block");
        Self::from(bytes)
    }
}

struct ProofOfWork {
    block: Block,
    target: BigInt,
}

const TARGET_BITS: i32 = 8;
const MAX_NONCE: i64 = i64::MAX;

impl ProofOfWork {
    fn new_proof_of_work(block: Block) -> ProofOfWork {
        let mut target = BigInt::from(1);

        target.shl_assign(256 - TARGET_BITS);
        ProofOfWork{block, target}
    }

    fn run(&self) -> (i64, String) {
        let mut nonce = 0;
       

       (nonce, String::new())
    }

}