use serde::Serialize;
use w3b_types_abi::{Address, Bytes32};

use super::block::BlockId;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Filter {
    #[serde(flatten)]
    pub blocks: FilterBlocks,
    pub address: Option<Address>,
    pub topics: Vec<Topic>,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum FilterBlocks {
    #[serde(rename_all = "camelCase")]
    Range {
        from_block: Option<BlockId>,
        to_block: Option<BlockId>,
    },
    Hash {
        blockhash: Bytes32,
    },
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum Topic {
    Hash(Bytes32),
    OneOf(Vec<Bytes32>),
}
