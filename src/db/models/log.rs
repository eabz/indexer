use alloy::rpc::types::Log;
use clickhouse::Row;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_as]
#[derive(Debug, Clone, Row, Serialize, Deserialize)]
pub struct DatabaseLog {
    pub address: String,
    pub block_number: u64,
    pub chain: u64,
    pub data: String,
    pub log_index: u64,
    pub removed: bool,
    pub timestamp: u64,
    pub topic0: String,
    pub topic1: Option<String>,
    pub topic2: Option<String>,
    pub topic3: Option<String>,
    pub transaction_hash: String,
    pub transaction_log_index: Option<u64>,
}

impl DatabaseLog {
    pub fn from_rpc(
        log: &Log,
        chain: u64,
        timestamp: u64,
        block_number: &u64,
    ) -> Self {
        let topic0 = if log.topic0().is_none() {
            String::from("0x")
        } else {
            log.topic0().unwrap().to_string()
        };

        let topics = log.topics();

        let topic1 = if topics.len() < 2 {
            None
        } else {
            Some(topics[1].to_string())
        };

        let topic2 = if topics.len() < 3 {
            None
        } else {
            Some(topics[2].to_string())
        };

        let topic3 = if topics.len() < 4 {
            None
        } else {
            Some(topics[3].to_string())
        };

        Self {
            address: log.address().to_string(),
            block_number: block_number.to_owned(),
            chain,
            data: log.data().data.to_string(),
            log_index: log.log_index.unwrap(),
            removed: log.removed,
            timestamp,
            topic0,
            topic1,
            topic2,
            topic3,
            transaction_hash: log.transaction_hash.unwrap().to_string(),
            transaction_log_index: log.transaction_index,
        }
    }
}
