use alloy::{
    primitives::{Address, U256},
    rpc::types::{Transaction, TransactionReceipt, TransactionTrait},
};
use clickhouse::Row;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_with::serde_as;

use crate::utils::format::{byte4_from_input, SerU256};

#[derive(Debug, Clone, Serialize_repr, Deserialize_repr, PartialEq)]
#[repr(u8)]
pub enum TransactionType {
    Legacy = 0,
    Eip2930 = 1,
    Eip1559 = 2,
    Eip4844 = 3,
    Eip7702 = 4,
}

#[serde_as]
#[derive(Debug, Clone, Row, Serialize, Deserialize)]
pub struct DatabaseTransaction {
    pub access_list: Vec<(String, Vec<String>)>,
    pub base_fee_per_gas: Option<u64>,
    pub block_hash: String,
    pub block_number: u64,
    pub chain: u64,
    pub contract_created: Option<String>,
    pub cumulative_gas_used: Option<u64>,
    pub effective_gas_price: Option<u128>,
    pub from: String,
    pub gas_limit: u64,
    pub gas_price: Option<u128>,
    pub gas_used: Option<u64>,
    pub hash: String,
    pub input: String,
    pub max_fee_per_gas: u128,
    pub max_priority_fee_per_gas: Option<u128>,
    pub method: String,
    pub nonce: u64,
    pub status: Option<bool>,
    pub timestamp: u64,
    pub to: String,
    pub transaction_index: u64,
    pub transaction_type: TransactionType,
    #[serde_as(as = "SerU256")]
    pub value: U256,
}

impl DatabaseTransaction {
    pub fn from_rpc(
        transaction: &Transaction,
        chain: u64,
        timestamp: u64,
    ) -> Self {
        let to: String = match transaction.to() {
            Some(address) => address.to_string(),
            None => Address::ZERO.to_string(),
        };

        let transaction_type = if transaction.inner.is_eip1559() {
            TransactionType::Eip1559
        } else if transaction.inner.is_eip2930() {
            TransactionType::Eip2930
        } else if transaction.inner.is_eip4844() {
            TransactionType::Eip4844
        } else if transaction.inner.is_eip7702() {
            TransactionType::Eip7702
        } else {
            TransactionType::Legacy
        };

        let access_list: Vec<(String, Vec<String>)> =
            match transaction.access_list() {
                Some(access_list_items) => {
                    let mut access_list: Vec<(String, Vec<String>)> =
                        Vec::new();

                    for item in access_list_items.to_vec() {
                        let keys: Vec<String> = item
                            .storage_keys
                            .into_iter()
                            .map(|item| item.to_string())
                            .collect();

                        access_list.push((item.address.to_string(), keys))
                    }

                    access_list
                }
                None => Vec::new(),
            };

        let input = transaction.input().to_string();

        Self {
            access_list,
            base_fee_per_gas: None,
            block_hash: transaction.block_hash.unwrap().to_string(),
            block_number: transaction.block_number.unwrap(),
            chain,
            contract_created: None,
            cumulative_gas_used: None,
            effective_gas_price: None,
            from: transaction.from.to_string(),
            gas_limit: transaction.gas_limit(),
            gas_price: transaction.gas_price(),
            gas_used: None,
            hash: transaction.inner.tx_hash().to_string(),
            input: input.clone(),
            max_fee_per_gas: transaction.max_fee_per_gas(),
            max_priority_fee_per_gas: transaction
                .max_priority_fee_per_gas(),
            method: format!("0x{}", hex::encode(byte4_from_input(&input))),
            nonce: transaction.nonce(),
            status: None,
            timestamp,
            to,
            transaction_index: transaction.transaction_index.unwrap(),
            transaction_type,
            value: transaction.value(),
        }
    }

    pub fn add_receipt_data(
        &mut self,
        base_fee_per_gas: Option<u64>,
        receipt: &TransactionReceipt,
    ) {
        self.base_fee_per_gas = base_fee_per_gas;
        self.contract_created =
            receipt.contract_address.map(|contract| contract.to_string());
        self.cumulative_gas_used =
            Some(receipt.inner.cumulative_gas_used());
        self.effective_gas_price = Some(receipt.effective_gas_price);
        self.gas_used = Some(receipt.gas_used);
        self.status = Some(receipt.status())
    }
}
