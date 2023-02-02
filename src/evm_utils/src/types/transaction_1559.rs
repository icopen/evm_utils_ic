use bytes::{BufMut, BytesMut};
use ic_cdk::export::candid::{CandidType, Deserialize};
use rlp::{Decodable, RlpStream};

use super::{
    address::Address,
    signature::{Signable, Signature},
};

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
    pub sign: Option<Signature>,
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

        let mut item = Self {
            chain_id,
            nonce,
            max_priority_fee_per_gas,
            max_fee_per_gas,
            gas_limit,
            to,
            value,
            data,
            access_list,
            sign: None,
        };

        if item_count == 12 {
            let mut buf = BytesMut::new();
            buf.extend_from_slice(&[2]);
            buf.extend_from_slice(rlp.as_raw());

            let signature = Signature::create(&item, rlp, &buf, 9)
                .map_err(|_| rlp::DecoderError::Custom("Error while recovering signature"))?;

            item.sign = Some(signature);
        }

        Ok(item)
    }
}

impl Signable for Transaction1559 {
    fn get_bytes(&self, for_signing: bool) -> bytes::BytesMut {
        let mut rlp = RlpStream::new();
        self.encode_rlp(&mut rlp, for_signing);

        let mut buf: BytesMut = BytesMut::new();
        buf.put_u8(2u8); //write EIP2930 identifier
        buf.extend_from_slice(rlp.out().as_ref());

        buf
    }

    fn encode_rlp(&self, rlp: &mut RlpStream, for_signing: bool) {
        rlp.begin_unbounded_list();

        rlp.append(&self.chain_id);
        rlp.append(&self.nonce);
        rlp.append(&self.max_priority_fee_per_gas);
        rlp.append(&self.max_fee_per_gas);
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
    fn decode_1559_transaction() -> Result<(), Box<dyn Error>> {
        let data =  "0x02f8b10108840f7f4900850519f0118083055845943fe65692bfcd0e6cf84cb1e7d24108e434a7587e80b8447050ccd90000000000000000000000005b1578681d43931030fffe066a072133842dde430000000000000000000000000000000000000000000000000000000000000001c001a0cac74d8e73874331e4ba78809a7b30574c76d35c43ea00983c76752b65a0632aa04b6caeac4c693f082b8bd4d2ad349848bc71b9d32a419e33a5a303a3b3f4e8c8";
        let data = hex::decode(data.trim_start_matches("0x"))?;

        let tx = Transaction::decode(&data)?;

        match tx {
            Transaction::EIP1559(x) => match x.sign {
                Some(sig) => {
                    assert_eq!(
                        "0x5b1578681d43931030fffe066a072133842dde43",
                        format!("{}", sig.from.unwrap())
                    );
                    assert_eq!(
                        "0x89f35f37f590d0f22b5ab8287bcd2d8a47683ef282b6ad4b2552ab01931faf36",
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
    fn encode_1559_transaction() -> Result<(), Box<dyn Error>> {
        let data_hex =  "0x02f8b10108840f7f4900850519f0118083055845943fe65692bfcd0e6cf84cb1e7d24108e434a7587e80b8447050ccd90000000000000000000000005b1578681d43931030fffe066a072133842dde430000000000000000000000000000000000000000000000000000000000000001c001a0cac74d8e73874331e4ba78809a7b30574c76d35c43ea00983c76752b65a0632aa04b6caeac4c693f082b8bd4d2ad349848bc71b9d32a419e33a5a303a3b3f4e8c8";
        let data = hex::decode(data_hex.trim_start_matches("0x"))?;

        let tx = Transaction::decode(&data)?;
        let encoded = tx.encode(false).to_vec();
        let encoded_hex = format!("0x{}", hex::encode(&encoded));

        assert_eq!(data, encoded);
        assert_eq!(data_hex, &encoded_hex);

        Ok(())
    }
}
