use super::*; //reach one level up the tree from your current location and import everything.
use std::fmt::{self, Debug, Formatter};

pub struct Block {
    pub index: u32,      //where does this lie in the blockchain
    pub timestamp: u128, //related to pub fn now ()
    pub hash: BlockHash, //BlockHash is a type allias
    pub prev_block_hash: BlockHash,
    pub nonce: u64,
    pub payload: String, //it would be better if it was a &str
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
    pub fn new(index: u32, timestamp: u128, prev_block_hash: BlockHash,nonce: u64, payload: String) -> Self {
        Block {
            index,
            timestamp,
            hash: vec![0; 32],
            prev_block_hash,
            nonce,
            payload,
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

        bytes
    }
}
