use serde::de::DeserializeOwned;
use w3b_types::*;

use crate::{error::Error, json_rpc::Response, namespace::Namespace, provider::Provider};

#[derive(Clone)]
pub struct Eth<T: Provider> {
    provider: T,
}

impl<T: Provider> Eth<T> {
    pub async fn block_number(&self) -> Result<u64, Error> {
        self.execute("eth_blockNumber", vec![])
            .await
            .map(Hex::inner)
    }

    pub async fn balance(
        &self,
        address: impl Into<Address>,
        block_number: impl Into<Option<BlockNumber>>,
    ) -> Result<Uint256, Error> {
        let address = serde_json::to_value(address.into()).unwrap();
        let block_number = serde_json::to_value(block_number.into().unwrap_or_default()).unwrap();
        self.execute("eth_getBalance", vec![address, block_number])
            .await
    }

    pub async fn logs(&self, filter: impl Into<Filter>) -> Result<Vec<Log>, Error> {
        let filter = serde_json::to_value(filter.into()).unwrap();
        self.execute("eth_getLogs", vec![filter]).await
    }

    pub async fn transaction_count(
        &self,
        address: impl Into<Address>,
        block_number: impl Into<Option<BlockNumber>>,
    ) -> Result<Uint64, Error> {
        let address = serde_json::to_value(address.into()).unwrap();
        let block_number = serde_json::to_value(block_number.into().unwrap_or_default()).unwrap();
        self.execute("eth_getTransactionCount", vec![address, block_number])
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

impl<T: Provider> Namespace<T> for Eth<T> {
    #[inline]
    fn new(provider: T) -> Self {
        Self { provider }
    }

    #[inline]
    fn provider(&self) -> &T {
        &self.provider
    }
}
