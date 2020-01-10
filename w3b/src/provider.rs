use std::{error::Error as StdError, future::Future};

use serde::de::DeserializeOwned;

use super::Request;

pub trait Provider {
    type Output: DeserializeOwned;
    type Error: StdError;
    type Response: Future<Output = Result<Self::Output, Self::Error>>;

    fn send(&self, request: Request) -> Self::Response;

    fn execute(&self, method: &str, params: Vec<serde_json::Value>) -> Self::Response {
        self.send(Request::new(method.to_owned(), params))
    }
}
