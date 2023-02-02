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

        // let mut buf: BytesMut = BytesMut::new();

        // let mut rlp = RlpStream::new();
        // match self {
        //     Transaction::Legacy(a) => a.encode_rlp(&mut rlp, for_signing),
        //     Transaction::EIP1559(a) => {
        //         buf.put_u8(2u8); //write EIP1559 identifier
        //         a.encode_rlp(&mut rlp, for_signing)
        //     },
        //     Transaction::EIP2930(a) => {
        //         buf.put_u8(1u8); //write EIP2930 identifier
        //         a.encode_rlp(&mut rlp, for_signing)
        //     }
        // }

        // buf.extend_from_slice(rlp.out().as_ref());

        // let data = match self {
        //     Transaction::Legacy(a) => (a as &dyn Signable).get_bytes(for_signing).to_vec(),
        //     Transaction::EIP1559(a) => {
        //         let mut buf: BytesMut = BytesMut::new();
        //         buf.put_u8(2u8); //write EIP1559 identifier
        //         buf.extend_from_slice(rlp::encode(a).as_ref());
        //         buf.to_vec()
        //     }
        //     Transaction::EIP2930(a) => {
        //         let mut buf: BytesMut = BytesMut::new();
        //         buf.put_u8(1u8); //write EIP2930 identifier
        //         // buf.extend_from_slice(rlp::encode(a).as_ref());
        //         buf.to_vec()
        //     }
        // };

        // Ok(buf.to_vec())
    }
}
