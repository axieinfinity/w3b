use w3b_types::Address;

mod error;
mod json_rpc;
mod provider;
pub mod providers;
mod web3;

pub use error::*;
pub use json_rpc::*;
pub use provider::*;
pub use web3::*;

use providers::HttpProvider;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let provider = HttpProvider::new(env!("JSON_RPC_URI").to_owned());
    let web3 = Web3::new(provider);

    web3.eth_block_number().await;

    web3.eth_transaction_count(
        // vitalik.eth
        Address::from_hex("0xd8da6bf26964af9d7eed9e03e53415d37aa96045").unwrap(),
        Some(9255000.into()),
    )
    .await;

    web3.eth_balance(
        // vitalik.eth
        Address::from_hex("0xd8da6bf26964af9d7eed9e03e53415d37aa96045").unwrap(),
        Some(9255000.into()),
    )
    .await;

    Ok(())
}
