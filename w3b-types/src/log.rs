use serde::Deserialize;
use w3b_types_abi::{Address, Bytes32, Uint64};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Log {
    pub address: Address,
    pub topics: Vec<Bytes32>,
    pub data: String,
    pub block_number: Option<Uint64>,
    pub block_hash: Option<Bytes32>,
    pub transaction_hash: Option<Bytes32>,
    pub transaction_index: Option<Uint64>,
    pub log_index: Option<Uint64>,
    pub transaction_log_index: Option<Uint64>,
    pub r#type: Option<String>,
    pub removed: bool,
}
