use reqwest::Client;

mod error;
mod json_rpc;
mod provider;
pub mod providers;

pub use error::*;
pub use json_rpc::*;
pub use provider::*;

use providers::HttpProvider;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::new();
    let provider = HttpProvider::new(client, env!("JSON_RPC_URI").to_owned());
    let value = provider.execute("eth_blockNumber", vec![]).await?;
    println!("{:?}", value);
    Ok(())
}
