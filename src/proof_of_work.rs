use crate::Block;
use data_encoding::HEXLOWER;
use num_bigint::{BigInt, Sign};
use std::borrow::Borrow;
use std::ops::ShlAssign;

const TARGET_BITS: i32 = 8;
const MAX_NONCE: i64 = i64::MAX;

// Basic Proof of Work implementation
pub struct ProofOfWork {
    block: Block,
    target: BigInt,
}

impl ProofOfWork {
    pub fn new_proof_of_work(block: Block) -> ProofOfWork {
        let mut target = BigInt::from(1);
        target.shl_assign(256 - TARGET_BITS);
        ProofOfWork { block, target }
    }
    
    fn prepare_data(&self, nonce: i64) -> Vec<u8> {
        let pre_block_hash = self.block.get_pre_block_hash();
        let transactions_hash = self.block.hash_transactions();
        let timestamp = self.block.get_timestamp();
        let mut data_bytes = vec![];
        data_bytes.extend(pre_block_hash.as_bytes());
        data_bytes.extend(transactions_hash);
        data_bytes.extend(timestamp.to_be_bytes());
        data_bytes.extend(TARGET_BITS.to_be_bytes());
        data_bytes.extend(nonce.to_be_bytes());
        return data_bytes;
    }
    
    pub fn run(&self) -> (i64, String) {
        let mut nonce = 0;
        let mut hash = Vec::new();
        println!("Mining the block");
        while nonce < MAX_NONCE {
            let data = self.prepare_data(nonce);
            hash = crate::sha256_digest(data.as_slice());
            let hash_int = BigInt::from_bytes_be(Sign::Plus, hash.as_slice());

            if hash_int.lt(self.target.borrow()) {
                println!("{}", HEXLOWER.encode(hash.as_slice()));
                break;
            } else {
                nonce += 1;
            }
        }
        println!();
        return (nonce, HEXLOWER.encode(hash.as_slice()));
    }
    
    pub fn validate(&self, nonce: i64) -> bool {
        // In a real implementation, this would validate the proof of work
        // using the provided nonce
        let hash = crate::sha256_digest(&nonce.to_be_bytes());
        let hash_int = BigInt::from_bytes_be(Sign::Plus, hash.as_slice());
        hash_int.lt(self.target.borrow())
    }
} 
