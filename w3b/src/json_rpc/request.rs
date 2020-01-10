use serde::Serialize;

use super::version::JsonRpcVersion;

#[derive(Serialize)]
pub struct Request {
    jsonrpc: JsonRpcVersion,
    id: serde_json::Value,
    method: String,
    params: Vec<serde_json::Value>,
}

impl Request {
    #[inline]
    pub fn new(method: String, params: Vec<serde_json::Value>) -> Self {
        Request {
            jsonrpc: JsonRpcVersion::V2,
            id: serde_json::Value::Null,
            method,
            params,
        }
    }
}
