use candid::export_service;
use ic_cdk::query;

mod hash;
mod rlp;
mod transaction;
mod tree;
mod types;
mod utils;

use crate::types::num::U256;
use crate::types::rlp::List;
use crate::types::transaction::Transaction;

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
