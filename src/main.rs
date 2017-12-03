extern crate chrono;
extern crate sha2;
extern crate byteorder;

use std::mem;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use chrono::prelude::*;
use sha2::{Sha256, Digest};
use byteorder::{BigEndian, WriteBytesExt};

fn main() {
    let mut bc = new_blockchain();

    // new block
    bc.new_transaction("me", "you", 5);
    bc.new_transaction("you", "me", 2);
    let proof = Blockchain::proof_of_work(bc.last_block().proof);
    bc.new_block(proof, None);

    println!("{:?}", bc);
}

#[derive(Debug)]
struct Blockchain {
    chain: Vec<Block>,
    current_transactions: Vec<Transaction>,
}

// Create an initialized blockchain.
fn new_blockchain() -> Blockchain {
    let mut bc = Blockchain { ..Default::default() };
    // add genesis block
    bc.new_block(100, Some(1));
    bc
}

impl Default for Blockchain {
    fn default() -> Blockchain {
        Blockchain {
            chain: Vec::new(),
            current_transactions: Vec::new(),
        }
    }
}

impl Blockchain {
    // Creates a new Block and adds it to the chain
    fn new_block(&mut self, proof: u64, previous_hash: Option<u64>) {
        let previous_hash = previous_hash.unwrap_or_else(|| Blockchain::hash(self.last_block()));

        let mut previous_transactions = Vec::new();
        mem::swap(&mut self.current_transactions, &mut previous_transactions);

        let block = Block {
            index: self.chain.len() + 1,
            timestamp: Utc::now(),
            transactions: previous_transactions,
            proof: proof,
            previous_hash: previous_hash,
        };

        self.chain.push(block);
    }

    // Adds a new transaction to the list of transactions
    fn new_transaction(&mut self, sender: &str, recipient: &str, amount: i64) -> usize {
        self.current_transactions.push(Transaction {
            sender: sender.to_owned(),
            recipient: recipient.to_owned(),
            amount: amount,
        });

        self.last_block().index + 1
    }

    // Hashes a Block
    fn hash(block: &Block) -> u64 {
        let mut s = DefaultHasher::new();
        block.hash(&mut s);
        s.finish()
    }

    // Returns the last Block in the chain
    fn last_block(&self) -> &Block {
        &self.chain[self.chain.len() - 1]
    }

    fn proof_of_work(last_proof: u64) -> u64 {
        let mut proof: u64 = 0;
        while Blockchain::valid_proof(last_proof, proof) == false {
            proof += 1;
        }
        proof
    }
    fn valid_proof(last_proof: u64, proof: u64) -> bool {
        let mut wtr = vec![];
        wtr.write_u64::<BigEndian>(last_proof).unwrap();
        wtr.write_u64::<BigEndian>(proof).unwrap();
        let mut hasher = Sha256::default();
        hasher.input(&wtr[..]);
        hasher.result()[..2] == b"00"[..2]
    }
}

#[derive(Hash, Debug)]
struct Block {
    index: usize,
    timestamp: DateTime<Utc>,
    transactions: Vec<Transaction>,
    proof: u64,
    previous_hash: u64,
}

#[derive(Hash, Debug)]
struct Transaction {
    sender: String,
    recipient: String,
    amount: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut bc = new_blockchain();
        assert_eq!(bc.chain.len(), 1);

        // new block
        bc.new_transaction("me", "you", 5);
        bc.new_transaction("you", "me", 2);
        assert_eq!(bc.current_transactions.len(), 2);

        let proof = Blockchain::proof_of_work(bc.last_block().proof);
        bc.new_block(proof, None);
        assert_eq!(bc.chain.len(), 2);
    }
}
