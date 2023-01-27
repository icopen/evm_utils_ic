use std::{
    error::Error,
    fmt::{self, Display}, vec,
};

use bytes::BufMut;
use bytes::BytesMut;
use ic_cdk::export::candid::{CandidType, Deserialize};
use rlp::{Decodable, Encodable, RlpStream, Rlp};
use secp256k1::{ecdsa::{RecoverableSignature, RecoveryId}, Message, PublicKey};
use sha3::digest::typenum::U2;

use crate::utils::{keccak256, _recover_public_key};

use super::{address::Address, num::U256};

#[derive(Debug)]
pub enum TransactionError {
    InvalidType,
}

impl std::error::Error for TransactionError {}

impl Display for TransactionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("{}", self))?;
        Ok(())
    }
}

#[derive(CandidType, Deserialize, Clone, PartialEq, Eq)]
pub enum Transaction {
    Legacy(TransactionLegacy),
    EIP1559(Transaction1559),
    EIP2930,
}

impl Transaction {
    pub fn decode(hex_raw_tx: &[u8]) -> Result<Transaction, Box<dyn Error>> {
        if hex_raw_tx[0] > 0x7f {
            Ok(Self::Legacy(rlp::decode(&hex_raw_tx)?))
        } else if hex_raw_tx[0] == 0x01 {
            Ok(Self::EIP2930)
        } else if hex_raw_tx[0] == 0x02 {
            Ok(Self::EIP1559(rlp::decode(&hex_raw_tx[1..])?))
        } else {
            Err(Box::new(TransactionError::InvalidType))
        }
    }

    pub fn encode(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        let data = match self {
            Transaction::Legacy(a) => rlp::encode(a).to_vec(),
            Transaction::EIP1559(a) => {
                let mut buf: BytesMut = BytesMut::new();
                buf.put_u8(2u8); //write EIP1559 identifier
                buf.extend_from_slice(rlp::encode(a).as_ref());
                buf.to_vec()
            }
            Transaction::EIP2930 => vec![],
        };

        Ok(data)
    }
}

#[derive(CandidType, Deserialize, Clone, PartialEq, Eq)]
pub struct TransactionLegacy {
    pub chain_id: u64,
    pub nonce: u64,
    pub gas_price: u64,
    pub gas_limit: u64,
    pub to: Address,
    pub value: u64,
    pub data: Vec<u8>,
    pub v: u64,
    pub r: Vec<u8>,
    pub s: Vec<u8>,
}

impl Decodable for TransactionLegacy {
    fn decode(rlp: &rlp::Rlp) -> Result<Self, rlp::DecoderError> {
        let item_count = rlp.item_count()?;
        if item_count != 6 && item_count != 9 {
            return Err(rlp::DecoderError::Custom(
                "Invalid parameters for legacy transaction",
            ));
        }

        let nonce: u64 = rlp.val_at(0)?;

        let gas_price: u64 = rlp.val_at(1)?;
        let gas_limit: u64 = rlp.val_at(2)?;

        let to = rlp.val_at(3)?;

        let value: u64 = rlp.val_at(4)?;
        let data: Vec<u8> = rlp.val_at(5)?;

        let mut chain_id = 0;

        let mut v = 0u64;
        let mut r: Vec<u8> = vec![];
        let mut s: Vec<u8> = vec![];

        // let mut from = None;

        if item_count == 9 {
            v= rlp.val_at(6)?;
            r = rlp.val_at(7)?;
            s = rlp.val_at(8)?;

            if v >= 35 {
                chain_id = (v-35)/2;
            }

            let mut rlp = RlpStream::new();
            rlp.begin_list(9);
            rlp.append(&nonce);
            rlp.append(&gas_price);
            rlp.append(&gas_limit);
            rlp.append(&to);
            rlp.append(&value);
            rlp.append(&data);
            rlp.append(&chain_id);
            rlp.append(&"");
            rlp.append(&"");
    

            let sender = _recover_public_key(&r, &s, v, &rlp.out()[..])
            .map_err(|_| rlp::DecoderError::Custom("Error decoding sender address"))?;

            // from = Some(Address::from(sender));
        }

        // let hash = keccak256(&[rlp.as_raw()]);

        let item = Self {
            chain_id,
            nonce,
            gas_price,
            gas_limit,
            to: to,
            value: value,
            data: data,
            v,
            r,
            s,
        };

        Ok(item)
    }
}

impl Encodable for TransactionLegacy {
    fn rlp_append(&self, rlp: &mut RlpStream) {
        rlp.begin_list(9);
        rlp.append(&self.nonce);
        rlp.append(&self.gas_price);
        rlp.append(&self.gas_limit);
        rlp.append(&self.to);
        rlp.append(&self.value);
        rlp.append(&self.data);

        rlp.append(&self.v);
        rlp.append(&self.r);
        rlp.append(&self.s);
    }
}

#[derive(CandidType, Deserialize, Clone, PartialEq, Eq)]
pub struct Transaction1559 {
    pub chain_id: u64,
    pub nonce: u64,
    pub max_priority_fee_per_gas: u64,
    pub gas_limit: u64,
    pub max_fee_per_gas: u64,
    pub to: Address,
    pub value: u64,
    pub data: Vec<u8>,
    pub access_list: Vec<u8>,
    pub v: u64,
    pub r: Vec<u8>,
    pub s: Vec<u8>,
    pub from: Option<Address>
}

impl Decodable for Transaction1559 {
    fn decode(rlp: &rlp::Rlp) -> Result<Self, rlp::DecoderError> {
        let item_count = rlp.item_count()?;
        if item_count != 9 && item_count != 12 {
            return Err(rlp::DecoderError::Custom(
                "Invalid parameters for 1559 transaction",
            ));
        }

        let chain_id: u64 = rlp.val_at(0)?;
        let nonce: u64 = rlp.val_at(1)?;

        let max_priority_fee_per_gas: u64 = rlp.val_at(2)?;
        let max_fee_per_gas: u64 = rlp.val_at(3)?;
        let gas_limit: u64 = rlp.val_at(4)?;

        let to = rlp.val_at(5)?;

        let value: u64 = rlp.val_at(6)?;
        let data: Vec<u8> = rlp.val_at(7)?;
        let access_list: Vec<u8> = rlp.at(8)?.as_raw().to_vec();

        let mut from = None;

        let mut v = 0;
        let mut r = vec![];
        let mut s = vec![];

        if item_count == 12 {
            v = rlp.val_at(9)?;
            r = rlp.val_at(10)?;
            s = rlp.val_at(11)?;

            let mut rlp = RlpStream::new();
            rlp.begin_list(9);
            rlp.append(&chain_id);
            rlp.append(&nonce);
            rlp.append(&max_priority_fee_per_gas);
            rlp.append(&max_fee_per_gas);
            rlp.append(&gas_limit);
            rlp.append(&to);
            rlp.append(&value);
            rlp.append(&data);
            rlp.append_raw(&access_list, access_list.len());
    
            let mut buf = BytesMut::new();
            buf.extend_from_slice(&[2]);
            buf.extend_from_slice(&rlp.out()[..]);

            let sender = _recover_public_key(&r, &s, v, &buf[..])
            .map_err(|_| rlp::DecoderError::Custom("Error decoding sender address"))?;

            from = Some(Address::from(sender));

        }

        let item = Self {
            chain_id,
            nonce,
            max_priority_fee_per_gas,
            max_fee_per_gas,
            gas_limit,
            to: to,
            value: value,
            data: data,
            access_list: access_list,
            v,
            r,
            s,
            from
        };

        Ok(item)
    }
}

impl Encodable for Transaction1559 {
    fn rlp_append(&self, rlp: &mut RlpStream) {
        rlp.begin_list(12);
        rlp.append(&self.chain_id);
        rlp.append(&self.nonce);
        rlp.append(&self.max_priority_fee_per_gas);
        rlp.append(&self.max_fee_per_gas);
        rlp.append(&self.gas_limit);
        rlp.append(&self.to);
        rlp.append(&self.value);
        rlp.append(&self.data);

        rlp.append_raw(&self.access_list, self.access_list.len());

        rlp.append(&self.v);
        rlp.append(&self.r);
        rlp.append(&self.s);
    }
}

#[cfg(test)]
mod test {
    use std::error::Error;

    use super::Transaction;

    #[test]
    fn decode_legacy_transaction() -> Result<(), Box<dyn Error>> {
        let data =  "f86e8302511e85036e1d083a826b6c948f2d10257ebf6386426456de1b1792b507426548875319b3e6ceb7bf8025a06716fc3c5bebebe88e61bc25714647b262904f7c99bd69c25541c7a796a9727fa071908b9fc3ce08f164cf1844ce43864a9347b7820a8921eef7aa67c55399e0be";
        let data = hex::decode(data)?;

        let tx = Transaction::decode(&data)?;
        match tx {
            Transaction::Legacy(x) => {
                assert_eq!(x.chain_id, 1);

                Ok(()) },
            _ => panic!("Wrong transaction type"),
        }
    }

    #[test]
    fn encode_legacy_transaction() -> Result<(), Box<dyn Error>> {
        let data_hex =  "f86d820144843b9aca0082520894b78777860637d56543da23312c7865024833f7d188016345785d8a0000802ba0e2539a5d9f056d7095bd19d6b77b850910eeafb71534ebd45159915fab202e91a007484420f3968697974413fc55d1142dc76285d30b1b9231ccb71ed1e720faae";
        let data = hex::decode(data_hex)?;

        let tx = Transaction::decode(&data)?;

        let encoded = Transaction::encode(&tx)?;
        let encoded_hex = hex::encode(&encoded);

        assert_eq!(data, encoded);
        assert_eq!(data_hex, &encoded_hex);

        Ok(())
    }

    #[test]
    fn decode_1559_transaction() -> Result<(), Box<dyn Error>> {
        let data =  "0x02f871015585037d46c34985037d46c34983027d0594e68df8dc3931aaab2077c57bbd2cbcedd17fcfce808457386225c080a0ac97a4d2f460d238fddaaed992047547d04c17ae454d6219a3a96699115ffcf5a006c2d3bd79f9b3321438721609ff6dddb0e50b8d9e38d02b68456590c33dee47";
        let data = hex::decode(data.trim_start_matches("0x"))?;

        let tx = Transaction::decode(&data)?;

        match tx {
            Transaction::EIP1559(x) => {
                match x.from {
                    Some(from) => println!("{}", from),
                    None => {}
                }

                Ok(())
            },
            _ => panic!("Wrong transaction type"),
        }
    }

    #[test]
    fn encode_1559_transaction() -> Result<(), Box<dyn Error>> {
        let data_hex =  "0x02f871015585037d46c34985037d46c34983027d0594e68df8dc3931aaab2077c57bbd2cbcedd17fcfce808457386225c080a0ac97a4d2f460d238fddaaed992047547d04c17ae454d6219a3a96699115ffcf5a006c2d3bd79f9b3321438721609ff6dddb0e50b8d9e38d02b68456590c33dee47";
        let data = hex::decode(data_hex.trim_start_matches("0x"))?;

        let tx = Transaction::decode(&data)?;

        let encoded = Transaction::encode(&tx)?;
        let encoded_hex = format!("0x{}", hex::encode(&encoded));

        assert_eq!(data, encoded);
        assert_eq!(data_hex, &encoded_hex);

        Ok(())
    }
}

// pub struct Transaction2930 {
//     pub chain_id: u64,
//     pub nonce: u64,
//     pub gas_price: u64,
//     pub gas_limit: u64,
//     pub to: String,
//     pub value: u64,
//     pub data: String,
//     pub access_list: Vec<(String, Vec<String>)>,
//     pub v: String,
//     pub r: String,
//     pub s: String,
// }
