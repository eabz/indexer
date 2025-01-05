use alloy::rpc::types::TransactionReceipt;
use clickhouse::Row;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Row, Serialize, Deserialize)]
pub struct DatabaseContract {
    pub block_number: u64,
    pub chain: u64,
    pub contract_address: String,
    pub creator: String,
    pub transaction_hash: String,
}

impl DatabaseContract {
    pub fn from_rpc(receipt: &TransactionReceipt, chain: u64) -> Self {
        Self {
            block_number: receipt.block_number.unwrap(),
            chain,
            contract_address: receipt
                .contract_address
                .unwrap()
                .to_string(),
            creator: receipt.from.to_string(),
            transaction_hash: receipt.transaction_hash.to_string(),
        }
    }
}
