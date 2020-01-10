use serde::de::DeserializeOwned;
use w3b_types::{Address, BlockId, BlockNumber, Uint256, Uint64};

use super::{error::Error, json_rpc::Response, provider::Provider};

pub struct Web3<T: Provider> {
    provider: T,
}

impl<T: Provider> Web3<T> {
    pub fn new(provider: T) -> Self {
        Self { provider }
    }
}

impl<T: Provider> Web3<T> {
    pub async fn eth_block_number(&self) -> Result<BlockNumber, Error> {
        self.execute("eth_blockNumber", vec![]).await
    }

    pub async fn eth_balance(
        &self,
        address: impl Into<Address>,
        block_id: impl Into<Option<BlockId>>,
    ) -> Result<Uint256, Error> {
        let address = serde_json::to_value(address.into()).unwrap();
        let block_id = serde_json::to_value(block_id.into().unwrap_or_default()).unwrap();
        self.execute("eth_getBalance", vec![address, block_id])
            .await
    }

    pub async fn eth_transaction_count(
        &self,
        address: impl Into<Address>,
        block_id: impl Into<Option<BlockId>>,
    ) -> Result<Uint64, Error> {
        let address = serde_json::to_value(address.into()).unwrap();
        let block_id = serde_json::to_value(block_id.into().unwrap_or_default()).unwrap();
        self.execute("eth_getTransactionCount", vec![address, block_id])
            .await
    }

    async fn execute<U: DeserializeOwned>(
        &self,
        method: &str,
        params: Vec<serde_json::Value>,
    ) -> Result<U, Error> {
        let value = self.provider.execute(method, params).await?;
        let response: Response = serde_json::from_value(value)?;
        let result = serde_json::from_value(response.result)?;
        Ok(result)
    }
}
