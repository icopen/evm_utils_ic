use super::{address::Address, num::U256};
use crate::utils::{_recover_public_key, keccak256};
use bytes::BytesMut;
use ic_cdk::export::candid::{CandidType, Deserialize};
use std::{error::Error, vec};

#[derive(CandidType, Deserialize, Clone, PartialEq, Eq)]
pub struct Signature {
    pub v: u64,
    pub r: Vec<u8>,
    pub s: Vec<u8>,
    pub from: Option<Address>,
    pub hash: U256,
}

impl Signature {
    pub fn create(
        tx: &dyn Signable,
        rlp: &rlp::Rlp,
        position: usize,
    ) -> Result<Self, Box<dyn Error>> {
        let v: u64 = rlp.val_at(position)?;
        let r: Vec<u8> = rlp.val_at(position + 1)?;
        let s: Vec<u8> = rlp.val_at(position + 2)?;

        let bytes = tx.get_bytes_for_signing()?;

        let sender = _recover_public_key(&r, &s, v, &bytes)?;
        let from = Some(Address::from(sender));

        let hash = keccak256(&[rlp.as_raw()]);

        Ok(Self {
            from,
            v,
            r,
            s,
            hash,
        })
    }
}

pub trait Signable {
    fn get_bytes_for_signing(&self) -> Result<BytesMut, Box<dyn Error>>;
}
