use std::future::Future;

use super::{error::Error, json_rpc::Request};

pub trait Provider {
    type Response: Future<Output = Result<serde_json::Value, Error>>;

    fn send(&self, request: Request) -> Self::Response;

    #[inline]
    fn execute(&self, method: &str, params: Vec<serde_json::Value>) -> Self::Response {
        self.send(Request::new(method.to_owned(), params))
    }
}
