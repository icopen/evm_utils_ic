use ic_cdk_macros::query;

use crate::types::transaction::Transaction;

#[query]
fn create_transaction(data: Transaction) -> Result<Vec<u8>, String> {
    let raw = data.encode()
    .map_err(|x| format!("Error while decoding transaction {}", x))?;
    
    Ok(raw)
}

/// Parses hex transaction, supports Legacy, EIP1559, EIP2930
#[query]
fn parse_transaction(data: Vec<u8>) -> Result<Transaction, String> {
    let item = Transaction::decode(&data)
        .map_err(|x| format!("Error while decoding transaction {}", x))?;

    Ok(item)
}