use alloy::rpc::types::Withdrawal;
use clickhouse::Row;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_as]
#[derive(Debug, Clone, Row, Serialize, Deserialize)]
pub struct DatabaseWithdrawal {
    pub address: String,
    pub amount: u64,
    pub block_number: u64,
    pub chain: u64,
    pub timestamp: u64,
    pub validator_index: u64,
    pub withdrawal_index: u64,
}

impl DatabaseWithdrawal {
    pub fn from_rpc(
        withdrawal: &Withdrawal,
        chain: u64,
        block_number: u64,
        timestamp: u64,
    ) -> Self {
        Self {
            address: withdrawal.address.to_string(),
            amount: withdrawal.amount,
            block_number,
            chain,
            timestamp,
            validator_index: withdrawal.validator_index,
            withdrawal_index: withdrawal.index,
        }
    }
}
