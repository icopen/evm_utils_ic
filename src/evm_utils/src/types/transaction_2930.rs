use bytes::{BufMut, BytesMut};
use ic_cdk::export::candid::{CandidType, Deserialize};
use rlp::{Decodable, RlpStream};

use super::{
    address::Address,
    signature::{Signable, Signature},
};

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
    pub sign: Option<Signature>,
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

        let mut item = Self {
            chain_id,
            nonce,
            gas_price,
            gas_limit,
            to,
            value,
            data,
            access_list,
            sign: None,
        };

        if item_count == 11 {
            let mut buf = BytesMut::new();
            buf.extend_from_slice(&[1]);
            buf.extend_from_slice(rlp.as_raw());

            let signature = Signature::create(&item, rlp, &buf, 8)
                .map_err(|_| rlp::DecoderError::Custom("Error while recovering signature"))?;

            item.sign = Some(signature);
        }

        Ok(item)
    }
}

impl Signable for Transaction2930 {
    fn get_bytes(&self, for_signing: bool) -> bytes::BytesMut {
        let mut rlp = RlpStream::new();
        self.encode_rlp(&mut rlp, for_signing);

        let mut buf: BytesMut = BytesMut::new();
        buf.put_u8(1u8); //write EIP2930 identifier
        buf.extend_from_slice(rlp.out().as_ref());

        buf
    }

    fn encode_rlp(&self, rlp: &mut RlpStream, for_signing: bool) {
        rlp.begin_unbounded_list();

        rlp.append(&self.chain_id);
        rlp.append(&self.nonce);
        rlp.append(&self.gas_price);
        rlp.append(&self.gas_limit);
        rlp.append(&self.to);
        rlp.append(&self.value);
        rlp.append(&self.data);
        rlp.append_raw(&self.access_list, self.access_list.len());

        if !for_signing && self.sign.is_some() {
            if let Some(sign) = self.sign.as_ref() {
                rlp.append(&sign.v);
                rlp.append(&sign.r);
                rlp.append(&sign.s);
            }
        }

        rlp.finalize_unbounded_list();
    }
}

#[cfg(test)]
mod test {
    use crate::types::transaction::Transaction;
    use std::error::Error;

    #[test]
    fn decode_2930_transaction() -> Result<(), Box<dyn Error>> {
        let data =  "0x01f8ab010485032af8977482d91194bbbbca6a901c926f240b89eacb641d8aec7aeafd80b844095ea7b3000000000000000000000000000000000022d473030f116ddee9f6b43ac78ba3ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc001a039c31c2e8693b56c4dc7b07fc77e1bfa6dd74f0cf55450f15a3c3e8ce0e2540ca071b92c8319f7415ba131e07f5481a20388bfd234640b58ae46c94ab2a5c2b7ea";
        let data = hex::decode(data.trim_start_matches("0x"))?;

        let tx = Transaction::decode(&data)?;

        match tx {
            Transaction::EIP2930(x) => match x.sign {
                Some(sig) => {
                    assert_eq!(
                        "0xef9ae1e5329f145dfbc5a4c435601a1a22a41fd0",
                        format!("{}", sig.from.unwrap())
                    );
                    assert_eq!(
                        "0x50f38b90c94d32726648b36cacfafbf71a07e898fe1d1dbc692aa751cbf296cd",
                        format!("{}", sig.hash)
                    );
                    Ok(())
                }
                None => panic!("Missing signature"),
            },
            _ => panic!("Wrong transaction type"),
        }
    }

    #[test]
    fn encode_2930_transaction() -> Result<(), Box<dyn Error>> {
        let data_hex =  "0x01f8ab010485032af8977482d91194bbbbca6a901c926f240b89eacb641d8aec7aeafd80b844095ea7b3000000000000000000000000000000000022d473030f116ddee9f6b43ac78ba3ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc001a039c31c2e8693b56c4dc7b07fc77e1bfa6dd74f0cf55450f15a3c3e8ce0e2540ca071b92c8319f7415ba131e07f5481a20388bfd234640b58ae46c94ab2a5c2b7ea";
        let data = hex::decode(data_hex.trim_start_matches("0x"))?;

        let tx = Transaction::decode(&data)?;

        let encoded = tx.encode(false);
        let encoded_hex = format!("0x{}", hex::encode(&encoded));

        assert_eq!(data, encoded);
        assert_eq!(data_hex, &encoded_hex);

        Ok(())
    }
}
