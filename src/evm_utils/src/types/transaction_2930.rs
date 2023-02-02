use ic_cdk::export::candid::{CandidType, Deserialize};
use rlp::{Decodable, Encodable, RlpStream};

use super::address::Address;

#[derive(CandidType, Deserialize, Clone, PartialEq, Eq)]
pub struct Transaction2930 {
    pub chain_id: u64,
    pub nonce: u64,
    pub gas_price: u64,
    pub gas_limit: u64,
    pub to: Address,
    pub value: u64,
    pub data: Vec<u8>,
    pub access_list: Vec<u8>,
}

impl Decodable for Transaction2930 {
    fn decode(rlp: &rlp::Rlp) -> Result<Self, rlp::DecoderError> {
        let item_count = rlp.item_count()?;
        if item_count != 8 && item_count != 11 {
            return Err(rlp::DecoderError::Custom(
                "Invalid parameters for 2930 transaction",
            ));
        }

        let chain_id: u64 = rlp.val_at(0)?;
        let nonce: u64 = rlp.val_at(1)?;

        let gas_price: u64 = rlp.val_at(2)?;
        let gas_limit: u64 = rlp.val_at(3)?;

        let to = rlp.val_at(4)?;

        let value: u64 = rlp.val_at(5)?;
        let data: Vec<u8> = rlp.val_at(6)?;
        let access_list: Vec<u8> = rlp.at(7)?.as_raw().to_vec();

        if item_count == 11 {
            // v = rlp.val_at(9)?;
            // r = rlp.val_at(10)?;
            // s = rlp.val_at(11)?;

            // let mut rlp = RlpStream::new();
            // rlp.begin_list(9);
            // rlp.append(&chain_id);
            // rlp.append(&nonce);
            // rlp.append(&max_priority_fee_per_gas);
            // rlp.append(&max_fee_per_gas);
            // rlp.append(&gas_limit);
            // rlp.append(&to);
            // rlp.append(&value);
            // rlp.append(&data);
            // rlp.append_raw(&access_list, access_list.len());

            // let mut buf = BytesMut::new();
            // buf.extend_from_slice(&[2]);
            // buf.extend_from_slice(&rlp.out()[..]);

            // let sender = _recover_public_key(&r, &s, v, &buf[..])
            //     .map_err(|_| rlp::DecoderError::Custom("Error decoding sender address"))?;

            // from = Some(Address::from(sender));
        }

        let item = Self {
            chain_id,
            nonce,
            gas_price,
            gas_limit,
            to,
            value,
            data,
            access_list,
        };

        Ok(item)
    }
}

impl Encodable for Transaction2930 {
    fn rlp_append(&self, rlp: &mut RlpStream) {
        rlp.begin_list(12);
        rlp.append(&self.chain_id);
        rlp.append(&self.nonce);
        rlp.append(&self.gas_price);
        rlp.append(&self.gas_limit);
        rlp.append(&self.to);
        rlp.append(&self.value);
        rlp.append(&self.data);

        rlp.append_raw(&self.access_list, self.access_list.len());
    }
}

#[cfg(test)]
mod test {
    use crate::types::transaction::Transaction;
    use std::error::Error;

    #[test]
    fn decode_1559_transaction() -> Result<(), Box<dyn Error>> {
        let data =  "0x02f871015585037d46c34985037d46c34983027d0594e68df8dc3931aaab2077c57bbd2cbcedd17fcfce808457386225c080a0ac97a4d2f460d238fddaaed992047547d04c17ae454d6219a3a96699115ffcf5a006c2d3bd79f9b3321438721609ff6dddb0e50b8d9e38d02b68456590c33dee47";
        let data = hex::decode(data.trim_start_matches("0x"))?;

        let tx = Transaction::decode(&data)?;

        match tx {
            Transaction::EIP1559(_) => Ok(()),
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
