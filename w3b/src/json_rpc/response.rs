use serde::Deserialize;

use super::version::JsonRpcVersion;

#[derive(Deserialize)]
pub struct Response {
    pub jsonrpc: JsonRpcVersion,
    pub id: serde_json::Value,
    pub result: serde_json::Value,
}
