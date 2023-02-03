use candid::candid_method;
use ic_cdk_macros::query;

use crate::{
    types::{num::U256, transaction::Transaction},
    utils::keccak256,
};

/// Encodes transaction in rlp, ready to be signed
#[query]
#[candid_method(query)]
fn create_transaction(data: Transaction) -> Result<(Vec<u8>, U256), String> {
    let raw = data.encode(true);
    let hash = keccak256(&[&raw]);

    Ok((raw.to_vec(), hash))
}

/// Parses raw transaction, supports Legacy, EIP1559, EIP2930
#[query]
#[candid_method(query)]
fn parse_transaction(data: Vec<u8>) -> Result<Transaction, String> {
    let item =
        Transaction::decode(&data).map_err(|x| format!("Error while decoding transaction {x}"))?;

    Ok(item)
}
