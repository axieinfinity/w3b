use num_bigint::BigUint;
use w3b_types::{Address, BlockId};

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

    let block_number = web3.eth_block_number().await?;

    println!("{:?}", block_number);

    let nonce: u64 = web3
        .eth_transaction_count(
            // vitalik.eth
            Address::from_hex("0xd8da6bf26964af9d7eed9e03e53415d37aa96045").unwrap(),
            Some(block_number.into()),
        )
        .await?
        .into();

    println!("{}", nonce);

    let balance: BigUint = web3
        .eth_balance(
            // vitalik.eth
            Address::from_hex("0xd8da6bf26964af9d7eed9e03e53415d37aa96045").unwrap(),
            Some(BlockId::Latest),
        )
        .await?
        .into();

    println!("{}", balance);

    Ok(())
}
