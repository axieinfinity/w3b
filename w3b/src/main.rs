use num_bigint::BigUint;
use w3b_types::{Address, BlockId, Bytes32, Filter, FilterBlocks, Topic};

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

    /* let block_number = web3.eth_block_number().await?;

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

    println!("{}", balance); */

    let logs = web3
        .eth_logs(Filter {
            blocks: FilterBlocks::Range {
                from_block: Some(9258817_u64.into()),
                to_block: Some(9258826_u64.into()),
            },
            address: Some(Address::from_hex("0xf5b0a3efb8e8e4c201e2a935f110eaaf3ffecb8d").unwrap()),
            topics: vec![Topic::OneOf(vec![
                // Approval
                Bytes32::from_hex(
                    "0x8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925",
                )
                .unwrap(),
                // Transfer
                Bytes32::from_hex(
                    "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef",
                )
                .unwrap(),
            ])],
        })
        .await?;

    println!("{:?}", logs);

    Ok(())
}
