use candid::{candid_method, export_service};
use ic_cdk_macros::query;

use crate::types::num::U256;
use crate::types::rlp::List;
use crate::types::transaction::Transaction;

/// Encodes transaction in rlp, ready to be signed
#[query]
#[candid_method(query)]
fn create_transaction(data: Transaction) -> Result<Vec<u8>, String> {
    let raw = data.encode(true);

    Ok(raw.to_vec())
}

/// Parses raw transaction, supports Legacy, EIP1559, EIP2930
#[query]
#[candid_method(query)]
fn parse_transaction(data: Vec<u8>) -> Result<Transaction, String> {
    let item =
        Transaction::decode(&data).map_err(|x| format!("Error while decoding transaction {x}"))?;

    Ok(item)
}

#[query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    export_service!();
    __export_service()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_candid() {
        use std::env;
        use std::fs::write;
        use std::path::PathBuf;

        let dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        let dir = dir.parent().unwrap().parent().unwrap().join("candid");
        write(dir.join("utils.did"), export_candid()).expect("Write failed.");
    }
}
