use crate::{types::num::U256, utils};
use ic_cdk::query;

/// Returns hash of given data using keccak256
#[query]
fn keccak256(data: Vec<u8>) -> U256 {
    utils::keccak256(&[&data[..]])
}
