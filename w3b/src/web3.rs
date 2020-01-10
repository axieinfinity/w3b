use w3b_types::{Address, BlockId};

use super::provider::Provider;

pub struct Web3<T: Provider> {
    provider: T,
}

impl<T: Provider> Web3<T> {
    pub fn new(provider: T) -> Self {
        Self { provider }
    }
}

impl<T: Provider> Web3<T> {
    pub async fn eth_block_number(&self) {
        let response = self
            .provider
            .execute("eth_blockNumber", vec![])
            .await
            .unwrap();

        println!("{:?}", response);
    }

    pub async fn eth_balance(
        &self,
        address: impl Into<Address>,
        block_id: impl Into<Option<BlockId>>,
    ) {
        let address = serde_json::to_value(address.into()).unwrap();
        let block_id = serde_json::to_value(block_id.into().unwrap_or_default()).unwrap();

        let response = self
            .provider
            .execute("eth_getBalance", vec![address, block_id])
            .await
            .unwrap();

        println!("{:?}", response);
    }

    pub async fn eth_transaction_count(
        &self,
        address: impl Into<Address>,
        block_id: impl Into<Option<BlockId>>,
    ) {
        let address = serde_json::to_value(address.into()).unwrap();
        let block_id = serde_json::to_value(block_id.into().unwrap_or_default()).unwrap();

        let response = self
            .provider
            .execute("eth_getTransactionCount", vec![address, block_id])
            .await
            .unwrap();

        println!("{:?}", response);
    }
}
