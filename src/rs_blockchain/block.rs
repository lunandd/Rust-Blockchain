use chrono::prelude::*;

use sha2::Digest;

use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Debug, Deserialize, Serialize, Clone)]
/// Block struct.
/// Used in combination with other blocks and a the Blockchain class
pub struct Block {
    pub index: u32,
    pub transactions: Vec<String>,
    pub timestamp: String,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u32,
}

#[allow(dead_code)]
impl Block {
    /// Creates a new block
    pub fn new(block_index: u32, transaction_vec: Vec<String>, prev_hash: String) -> Self {
        let local_date_time: String = Local::now().format("%d-%m-%Y %H:%M:%S").to_string();
        Self {
            index: block_index,
            transactions: transaction_vec,
            timestamp: local_date_time,
            previous_hash: prev_hash,
            hash: String::from("0"),
            nonce: 0u32,
        }
    }

    /// Private function.
    /// Returns
    fn get_json_result(&self) -> Result<String> {
        let json_data = serde_json::to_string(&self)?;
        Ok(json_data)
    }

    pub fn get_json(&self) -> String {
        let result_json = self.get_json_result();
        result_json.as_ref().unwrap().to_string()
    }

    pub fn compute_hash(&mut self) -> String {
        // if self.transactions[0].is_empty() {
        //     self.transactions.remove(0);
        // }

        let json = self.get_json();
        format!("{:x}", sha2::Sha256::digest(json.as_bytes()))
    }
}