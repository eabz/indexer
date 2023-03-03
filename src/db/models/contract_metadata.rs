use field_count::FieldCount;

#[derive(Debug, Clone, FieldCount)]
pub struct DatabaseContractMetadata {
    pub abi: String,
    pub chain: i64,
    pub contract_address: String,
    pub name: String,
}
