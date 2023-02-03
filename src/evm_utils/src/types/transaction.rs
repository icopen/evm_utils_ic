use std::{error::Error, vec};

use bytes::BytesMut;
use ic_cdk::export::candid::{CandidType, Deserialize};

use super::errors::TransactionError;
use super::signature::Signable;
use super::transaction_1559::Transaction1559;
use super::transaction_2930::Transaction2930;
use super::transaction_legacy::TransactionLegacy;

#[derive(CandidType, Deserialize, Clone, PartialEq, Eq)]
pub enum Transaction {
    Legacy(TransactionLegacy),
    EIP1559(Transaction1559),
    EIP2930(Transaction2930),
}

impl Transaction {
    pub fn decode(hex_raw_tx: &[u8]) -> Result<Transaction, Box<dyn Error>> {
        if hex_raw_tx[0] > 0x7f {
            Ok(Self::Legacy(rlp::decode(hex_raw_tx)?))
        } else if hex_raw_tx[0] == 0x01 {
            Ok(Self::EIP2930(rlp::decode(&hex_raw_tx[1..])?))
        } else if hex_raw_tx[0] == 0x02 {
            Ok(Self::EIP1559(rlp::decode(&hex_raw_tx[1..])?))
        } else {
            Err(Box::new(TransactionError::InvalidType))
        }
    }

    pub fn encode(&self, for_signing: bool) -> BytesMut {
        match self {
            Transaction::Legacy(a) => a.get_bytes(for_signing),
            Transaction::EIP1559(a) => a.get_bytes(for_signing),
            Transaction::EIP2930(a) => a.get_bytes(for_signing),
        }
    }
}
