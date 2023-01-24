use std::{
    error::Error,
    fmt::{self, Display},
};

use ic_cdk::export::candid::{CandidType, Deserialize};
use rlp::{Decodable, Encodable, RlpStream};

use crate::utils::keccak256;

use super::num::U256;

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
    EIP1559,
    EIP2930,
}

impl Transaction {
    pub fn decode(hex_raw_tx: &[u8]) -> Result<Transaction, Box<dyn Error>> {
        if hex_raw_tx[0] > 0x7f {
            Ok(Self::Legacy(rlp::decode(&hex_raw_tx)?))
        } else if hex_raw_tx[0] == 0x01 {
            Ok(Self::EIP2930)
        } else if hex_raw_tx[0] == 0x02 {
            Ok(Self::EIP1559)
        } else {
            Err(Box::new(TransactionError::InvalidType))
        }
    }

    pub fn encode(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        let data = match self {
            Transaction::Legacy(a) => rlp::encode(a).to_vec(),
            Transaction::EIP1559 => vec![],
            Transaction::EIP2930 => vec![]
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
    pub to: String,
    pub value: u64,
    pub data: Vec<u8>,
    pub v: Vec<u8>,
    pub r: Vec<u8>,
    pub s: Vec<u8>,
    pub hash: U256,
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

        let to = hex::encode(rlp.val_at::<Vec<u8>>(3)?);

        let value: u64 = rlp.val_at(4)?;
        let data: Vec<u8> = rlp.val_at(5)?;

        let hash = keccak256(&[rlp.data()?]);

        let mut item = Self {
            chain_id: 0,
            nonce,
            gas_price,
            gas_limit,
            to: to,
            value: value,
            data: data,
            v: vec![],
            r: vec![],
            s: vec![],
            hash,
        };

        if item_count == 9 {
            let v: Vec<u8> = rlp.val_at(6)?;
            let r: Vec<u8> = rlp.val_at(7)?;
            let s: Vec<u8> = rlp.val_at(8)?;

            item.v = v;
            item.r = r;
            item.s = s;

            // if r == 0 && s == 0 {
            //     item.chain_id = v;
            //     item.v = 0;
            // }
        }

        Ok(item)
    }
}

impl Encodable for TransactionLegacy {
    fn rlp_append(&self, rlp: &mut RlpStream) {
        // rlp.append_list(&self.values);
    }
}

#[cfg(test)]
mod test {
    use std::error::Error;

    use super::{Transaction};

    #[test]
    fn decode_legacy_transaction() -> Result<(), Box<dyn Error>> {
        let data =  "f86d820144843b9aca0082520894b78777860637d56543da23312c7865024833f7d188016345785d8a0000802ba0e2539a5d9f056d7095bd19d6b77b850910eeafb71534ebd45159915fab202e91a007484420f3968697974413fc55d1142dc76285d30b1b9231ccb71ed1e720faae";
        let data = hex::decode(data)?;

        Transaction::decode(&data)?;

        Ok(())
    }
}

// pub struct Transaction1559 {
//     pub chain_id: u64,
//     pub nonce: u64,
//     pub max_priority_fee_per_gas: u64,
//     pub gas_limit: u64,
//     pub max_fee_per_gas: u64,
//     pub to: String,
//     pub value: u64,
//     pub data: String,
//     pub access_list: Vec<(String, Vec<String>)>,
//     pub v: String,
//     pub r: String,
//     pub s: String,
// }

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

// impl From<(Vec<u8>, u64)> for TransactionLegacy {
//     fn from(data: (Vec<u8>, u64)) -> Self {
//         let rlp = rlp::Rlp::new(&data.0[..]);

//         let nonce_hex = rlp.at(0).as_val::<Vec<u8>>();
//         let nonce = vec_u8_to_u64(&nonce_hex);

//         let gas_price_hex = rlp.at(1).as_val::<Vec<u8>>();
//         let gas_price = vec_u8_to_u64(&gas_price_hex);

//         let gas_limit_hex = rlp.at(2).as_val::<Vec<u8>>();
//         let gas_limit = vec_u8_to_u64(&gas_limit_hex);

//         let to_hex = rlp.at(3).as_val::<Vec<u8>>();
//         let to = vec_u8_to_string(&to_hex);

//         let value_hex = rlp.at(4).as_val::<Vec<u8>>();
//         let value = vec_u8_to_u64(&value_hex);

//         let data_tx_hex = rlp.at(5).as_val::<Vec<u8>>();
//         let data_tx = vec_u8_to_string(&data_tx_hex);

//         let v_hex = rlp.at(6).as_val::<Vec<u8>>();
//         let v = vec_u8_to_string(&v_hex);

//         let r_hex = rlp.at(7).as_val::<Vec<u8>>();
//         let r = vec_u8_to_string(&r_hex);

//         let s_hex = rlp.at(8).as_val::<Vec<u8>>();
//         let s = vec_u8_to_string(&s_hex);

//         let chain_id =data.1;

//         TransactionLegacy {
//             chain_id,
//             nonce,
//             gas_price,
//             gas_limit,
//             to,
//             value,
//             data: data_tx,
//             v,
//             r,
//             s,
//         }
//     }
// }

// impl From<Vec<u8>> for Transaction2930 {
//     fn from(data: Vec<u8>) -> Self {
//         let rlp = rlp::Rlp::new(&data[1..]);

//         let chain_id_hex = rlp.at(0).as_val::<Vec<u8>>();
//         let chain_id = vec_u8_to_u64(&chain_id_hex);

//         let nonce_hex = rlp.at(1).as_val::<Vec<u8>>();
//         let nonce = vec_u8_to_u64(&nonce_hex);

//         let gas_price_hex = rlp.at(2).as_val::<Vec<u8>>();
//         let gas_price = vec_u8_to_u64(&gas_price_hex);

//         let gas_limit_hex = rlp.at(3).as_val::<Vec<u8>>();
//         let gas_limit = vec_u8_to_u64(&gas_limit_hex);

//         let to_hex = rlp.at(4).as_val::<Vec<u8>>();
//         let to = vec_u8_to_string(&to_hex);

//         let value_hex = rlp.at(5).as_val::<Vec<u8>>();
//         let value = vec_u8_to_u64(&value_hex);

//         let data_tx_hex = rlp.at(6).as_val::<Vec<u8>>();
//         let data_tx = vec_u8_to_string(&data_tx_hex);

//         let access_list = decode_access_list(&rlp.at(7).as_raw().to_vec());

//         let v_hex = rlp.at(8).as_val::<Vec<u8>>();
//         let v = vec_u8_to_string(&v_hex);

//         let r_hex = rlp.at(9).as_val::<Vec<u8>>();
//         let r = vec_u8_to_string(&r_hex);

//         let s_hex = rlp.at(10).as_val::<Vec<u8>>();
//         let s = vec_u8_to_string(&s_hex);
//         Transaction2930 {
//             chain_id,
//             nonce,
//             gas_price,
//             gas_limit,
//             to,
//             data: data_tx,
//             value,
//             access_list,
//             v,
//             r,
//             s,
//         }

//     }
// }

// impl From<Vec<u8>> for Transaction1559 {
//     fn from(data: Vec<u8>) -> Self {
//         let rlp = rlp::Rlp::new(&data[1..]);

//         // let chain_id_hex = rlp.at(0).as_val::<Vec<u8>>();
//         // let chain_id = vec_u8_to_u64(&chain_id_hex);

//         // let nonce_hex = rlp.at(1).as_val::<Vec<u8>>();
//         // let nonce = vec_u8_to_u64(&nonce_hex);

//         // let max_priority_fee_per_gas_hex = rlp.at(2).as_val::<Vec<u8>>();
//         // let max_priority_fee_per_gas = vec_u8_to_u64(&max_priority_fee_per_gas_hex);

//         // let max_fee_per_gas_hex = rlp.at(3).as_val::<Vec<u8>>();

//         // let max_fee_per_gas = vec_u8_to_u64(&max_fee_per_gas_hex);

//         // let gas_limit_hex = rlp.at(4).as_val::<Vec<u8>>();
//         // let gas_limit = vec_u8_to_u64(&gas_limit_hex);

//         // let to_hex = rlp.at(5).as_val::<Vec<u8>>();
//         // let to = vec_u8_to_string(&to_hex);

//         // let value_hex = rlp.at(6).as_val::<Vec<u8>>();
//         // let value = vec_u8_to_u64(&value_hex);

//         // let data_tx_hex = rlp.at(7).as_val::<Vec<u8>>();
//         // let data_tx = vec_u8_to_string(&data_tx_hex);

//         // let access_list = decode_access_list(&rlp.at(8).as_raw().to_vec());

//         // let v_hex = rlp.at(9).as_val::<Vec<u8>>();
//         // let v = vec_u8_to_string(&v_hex);

//         // let r_hex = rlp.at(10).as_val::<Vec<u8>>();
//         // let r = vec_u8_to_string(&r_hex);

//         // let s_hex = rlp.at(11).as_val::<Vec<u8>>();
//         // let s = vec_u8_to_string(&s_hex);

//         // Transaction1559 {
//         //     chain_id,
//         //     nonce,
//         //     max_priority_fee_per_gas,
//         //     max_fee_per_gas,
//         //     gas_limit,
//         //     to,
//         //     value,
//         //     data: data_tx,
//         //     access_list,
//         //     v,
//         //     r,
//         //     s,
//         // }
//     }
// }

// fn encode_access_list(access_list: &Vec<(String, Vec<String>)>) -> Vec<u8> {
//     let mut stream = rlp::RlpStream::new_list(access_list.len());

//     for list in access_list {
//         let mut stream_tuple = rlp::RlpStream::new_list(2);

//         // append address
//         stream_tuple.append(&string_to_vec_u8(&list.0[..]));

//         // append storage keys
//         let mut stream_storage_keys = rlp::RlpStream::new_list(list.1.len());
//         for storage_key in list.1.clone() {
//             stream_storage_keys.append(&string_to_vec_u8(&storage_key[..]));
//         }
//         stream_tuple.append_raw(&stream_storage_keys.out(), 1);

//         // append (address, storage_keys)
//         stream.append_raw(&stream_tuple.out(), 1);
//     }

//     stream.out().to_vec()
// }

// fn decode_access_list(access_list: &Vec<u8>) -> Vec<(String, Vec<String>)> {
//     let mut decoded_access_list = vec![];
//     let rlp = rlp::Rlp::new(access_list);
//     for item in rlp.iter() {
//         let address = item.at(0).as_val();
//         let storage_keys_u8 = item.at(1).as_list::<Vec<u8>>();
//         let storage_keys = storage_keys_u8
//             .iter()
//             .map(|x| vec_u8_to_string(x))
//             .collect::<Vec<String>>();
//         decoded_access_list.push((vec_u8_to_string(&address), storage_keys));
//     }
//     decoded_access_list
// }
