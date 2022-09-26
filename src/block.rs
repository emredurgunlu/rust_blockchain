use super::*; //reach one level up the tree from your current location and import everything.
use std::fmt::{self, Debug, Formatter};

pub struct Block {
    pub index: u32,      //where does this lie in the blockchain
    pub timestamp: u128, //related to pub fn now ()
    pub hash: BlockHash, //BlockHash is a type allias
    pub prev_block_hash: BlockHash,
    pub nonce: u64,
    pub payload: String, //it would be better if it was a &str
    pub difficulty: u128,
}
impl Debug for Block {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Block[{}]: {} at: {} with: {} nonce: {}",
            &self.index,
            &hex::encode(&self.hash),
            &self.timestamp,
            &self.payload,
            &self.nonce,
        )
    }
}
impl Block {
    pub fn new(
        index: u32,
        timestamp: u128,
        prev_block_hash: BlockHash,
        nonce: u64,
        payload: String,
        difficulty: u128,
    ) -> Self {
        Block {
            index,
            timestamp,
            hash: vec![0; 32],
            prev_block_hash,
            nonce,
            payload,
            difficulty,
        }
    }
    pub fn mine(&mut self) {
        for nonce_attempt in 0..(u64::max_value()) {
            self.nonce = nonce_attempt;
            let hash = self.hash();
            if check_difficulty(&hash, self.difficulty) {
                self.hash = hash;
                return;
            }
        }
    }
}

impl Hashable for Block {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend(&u32_bytes(&self.index));
        bytes.extend(&u128_bytes(&self.timestamp));
        bytes.extend(&self.prev_block_hash);
        bytes.extend(&u64_bytes(&self.nonce));
        bytes.extend(self.payload.as_bytes());
        bytes.extend(&u128_bytes(&self.difficulty));

        bytes
    }
}

pub fn check_difficulty(hash: &BlockHash, difficulty: u128) -> bool {
    difficulty > difficulty_bytes_as_u128(&hash)
} 
/* if the number of "difficulty" becomes smaller, then it becomes harder to get true from "fn check_difficulty".
In other words, if there are more zero at the beginning of the difficulty number, then it becomes harder to mine it and find the nonce value.
Before comparing the difficulty number and last 16 bytes of the hash, the last 16 bytes of the hash is flipped.
For example, if there are two zero at the beginning of the difficulty number, then there should be two zero at the end of the hash number.*/