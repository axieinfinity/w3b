use std::{future::Future, pin::Pin};

use reqwest::Client;

use crate::{error::Error, json_rpc::Request, provider::Provider};

pub struct HttpProvider {
    client: Client,
    uri: String,
}

impl HttpProvider {
    #[inline]
    pub fn new(uri: String) -> Self {
        Self::with_client(Client::new(), uri)
    }

    #[inline]
    pub fn with_client(client: Client, uri: String) -> Self {
        Self { client, uri }
    }
}

impl Provider for HttpProvider {
    type Response = Pin<Box<dyn Future<Output = Result<serde_json::Value, Error>>>>;

    fn send(&self, request: Request) -> Self::Response {
        let request = self.client.post(&self.uri).json(&request);

        Box::pin(async move {
            let response = request.send().await?;
            let bytes = response.bytes().await?;
            let value = serde_json::from_slice(&bytes)?;
            Ok(value)
        })
    }
}
