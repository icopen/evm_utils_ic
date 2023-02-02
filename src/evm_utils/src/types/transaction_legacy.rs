use std::error::Error;

use bytes::BytesMut;
use ic_cdk::export::candid::{CandidType, Deserialize};
use rlp::{Decodable, Encodable, RlpStream};

use super::{
    address::Address,
    signature::{Signable, Signature},
};

#[derive(CandidType, Deserialize, Clone, PartialEq, Eq)]
pub struct TransactionLegacy {
    pub chain_id: u64,
    pub nonce: u64,
    pub gas_price: u64,
    pub gas_limit: u64,
    pub to: Address,
    pub value: u64,
    pub data: Vec<u8>,
    pub sign: Option<Signature>,
}

impl Signable for TransactionLegacy {
    fn get_bytes_for_signing(&self) -> Result<BytesMut, Box<dyn Error>> {
        let mut rlp = RlpStream::new();
        rlp.begin_list(9);
        rlp.append(&self.nonce);
        rlp.append(&self.gas_price);
        rlp.append(&self.gas_limit);
        rlp.append(&self.to);
        rlp.append(&self.value);
        rlp.append(&self.data);
        rlp.append(&self.chain_id);
        rlp.append(&"");
        rlp.append(&"");
        Ok(rlp.out())
    }
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

        let mut item = Self {
            chain_id: 0,
            nonce,
            gas_price,
            gas_limit,
            to,
            value,
            data,
            sign: None,
        };

        if item_count == 9 {
            let v: u64 = rlp.val_at(6)?;

            let mut chain_id = 0;
            if v >= 35 {
                chain_id = (v - 35) / 2;
            }

            let signature = Signature::create(&item, rlp, 6)
                .map_err(|_| rlp::DecoderError::Custom("Error while recovering signature"))?;

            item.sign = Some(signature);
            item.chain_id = chain_id;
        }

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
    }
}

#[cfg(test)]
mod test {
    use std::error::Error;

    use crate::types::transaction::Transaction;

    #[test]
    fn decode_legacy_transaction() -> Result<(), Box<dyn Error>> {
        let data =  "f86e8302511e85036e1d083a826b6c948f2d10257ebf6386426456de1b1792b507426548875319b3e6ceb7bf8025a06716fc3c5bebebe88e61bc25714647b262904f7c99bd69c25541c7a796a9727fa071908b9fc3ce08f164cf1844ce43864a9347b7820a8921eef7aa67c55399e0be";
        let data = hex::decode(data)?;

        let tx = Transaction::decode(&data)?;
        match tx {
            Transaction::Legacy(x) => {
                assert_eq!(x.chain_id, 1);

                Ok(())
            }
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
}
