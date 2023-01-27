use ic_cdk_macros::query;

use crate::types::transaction::Transaction;

/// Encodes transaction in rlp, ready to be signed
#[query]
fn get_transaction_for_signing(data: Transaction) -> Result<Vec<u8>, String> {
    let raw = data
        .encode()
        .map_err(|x| format!("Error while encoding transaction {}", x))?;

    Ok(raw)
}

/// Converts transaction to a RAW format, returns encoded transaction and its hash
fn create_signed_transaction(data: Transaction) -> Result<Vec<u8>, String> {
    Err("not implemented".to_string())
}

/// Parses raw transaction, supports Legacy, EIP1559, EIP2930
#[query]
fn parse_transaction(data: Vec<u8>) -> Result<Transaction, String> {
    let item = Transaction::decode(&data)
        .map_err(|x| format!("Error while decoding transaction {}", x))?;

    Ok(item)
}
