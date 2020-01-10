use std::{error::Error as StdError, future::Future};

use super::json_rpc::Request;

pub trait Provider {
    type Error: StdError;
    type Response: Future<Output = Result<serde_json::Value, Self::Error>>;

    fn send(&self, request: Request) -> Self::Response;

    fn execute(&self, method: &str, params: Vec<serde_json::Value>) -> Self::Response {
        self.send(Request::new(method.to_owned(), params))
    }
}
