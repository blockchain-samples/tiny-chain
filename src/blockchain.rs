extern crate time;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate sha2;

use sha2::{Sha256, Digest};
use std::fmt::Write;

#[derive(Serialize, Debug, Clone)]
struct Transaction {
    sender: String,
    receiver: String,
    amount: f32,
}

#[derive(Debug, Serialize)]
pub struct Blockheader {
    timestamp: i64,
    nonce: u32,
    pre_hash: String,
    merkle: String,
    difficulty: u32,
}

pub struct Block {
    header: Blockheader,
    count: u32,
    transaction: Vec<Transaction>
}

pub struct Chain {
    chain: Vec<Block>,
    curr_trans: Vec<Transaction>,
    difficulty: u32,
    miner_addr: String,
    reward: f32,
}

impl Chain {
    pub fn new(miner_addr: String, difficulty: u32) -> Chain {
        let mut chain = Chain {
            chain: Vec::new(),
            curr_trans: Vec::new(),
            difficulty,
            miner_addr,
            reward: 100.0,
        };

        chain.generate_new_block();
        chain
    }
}