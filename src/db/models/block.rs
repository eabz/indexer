use alloy::{primitives::U256, rpc::types::Block};
use clickhouse::Row;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::utils::format::SerU256;

#[serde_as]
#[derive(Debug, Clone, Row, Serialize, Deserialize)]
pub struct DatabaseBlock {
    pub base_fee_per_gas: Option<u64>,
    pub chain: u64,
    #[serde_as(as = "SerU256")]
    pub difficulty: U256,
    pub extra_data: String,
    pub gas_limit: u64,
    pub gas_used: u64,
    pub hash: String,
    pub is_uncle: bool,
    pub logs_bloom: String,
    pub miner: String,
    pub mix_hash: String,
    pub nonce: String,
    pub number: u64,
    pub parent_hash: String,
    pub receipts_root: String,
    pub sha3_uncles: String,
    pub size: u32,
    pub state_root: String,
    pub timestamp: u64,
    #[serde_as(as = "Option<SerU256>")]
    pub total_difficulty: Option<U256>,
    pub transactions: u16,
    pub transactions_root: String,
    pub uncles: Vec<String>,
    pub withdrawals_root: Option<String>,
}

impl DatabaseBlock {
    pub fn from_rpc<T>(
        block: &Block<T>,
        chain: u64,
        is_uncle: bool,
    ) -> Self {
        let withdrawals_root: Option<String> = block
            .header
            .withdrawals_root
            .map(|withdrawals_root| withdrawals_root.to_string());

        let base_fee_per_gas: Option<u64> = block
            .header
            .base_fee_per_gas
            .map(|base_fee_per_gas| base_fee_per_gas);

        Self {
            base_fee_per_gas,
            chain,
            difficulty: block.header.difficulty,
            extra_data: block.header.extra_data.to_string(),
            gas_limit: block.header.gas_limit,
            gas_used: block.header.gas_used,
            hash: block.header.hash.to_string(),
            is_uncle,
            logs_bloom: block.header.logs_bloom.to_string(),
            miner: block.header.beneficiary.to_string(),
            mix_hash: block.header.mix_hash.to_string(),
            nonce: block.header.nonce.to_string(),
            number: block.header.number,
            parent_hash: block.header.parent_hash.to_string(),
            receipts_root: block.header.receipts_root.to_string(),
            sha3_uncles: block.header.ommers_hash.to_string(),
            size: block
                .header
                .size
                .unwrap()
                .to_string()
                .parse::<u32>()
                .unwrap(),
            state_root: block.header.state_root.to_string(),
            timestamp: block.header.timestamp,
            total_difficulty: block.header.total_difficulty,
            transactions: block.transactions.len() as u16,
            transactions_root: block.header.transactions_root.to_string(),
            uncles: block
                .uncles
                .clone()
                .into_iter()
                .map(|uncle| uncle.to_string())
                .collect(),
            withdrawals_root,
        }
    }
}
