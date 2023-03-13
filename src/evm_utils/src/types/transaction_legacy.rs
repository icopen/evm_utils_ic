use bytes::BytesMut;
use ic_cdk::export::candid::{CandidType, Deserialize};
use rlp::{Decodable, RlpStream};

use super::{
    address::Address,
    num::U256,
    signature::{Signable, Signature},
};

#[derive(CandidType, Deserialize, Clone, PartialEq, Eq)]
pub struct TransactionLegacy {
    pub chain_id: u64,
    pub nonce: U256,
    pub gas_price: U256,
    pub gas_limit: U256,
    pub to: Address,
    pub value: U256,
    pub data: Vec<u8>,
    pub sign: Option<Signature>,
}

impl Decodable for TransactionLegacy {
    fn decode(rlp: &rlp::Rlp) -> Result<Self, rlp::DecoderError> {
        let item_count = rlp.item_count()?;
        if item_count != 6 && item_count != 9 {
            return Err(rlp::DecoderError::Custom(
                "Invalid parameters for legacy transaction",
            ));
        }

        let nonce: U256 = rlp.val_at(0)?;

        let gas_price: U256 = rlp.val_at(1)?;
        let gas_limit: U256 = rlp.val_at(2)?;

        let to = rlp.val_at(3)?;

        let value: U256 = rlp.val_at(4)?;
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
            let r: Vec<u8> = rlp.val_at(7)?;

            if v >= 35 {
                item.chain_id = (v - 35) / 2;
            } else {
                item.chain_id = v;
            }

            if !r.is_empty() {
                let signature = Signature::create(&item, rlp, rlp.as_raw(), 6)
                    .map_err(|_| rlp::DecoderError::Custom("Error while recovering signature"))?;

                item.sign = Some(signature);
            }
        }

        Ok(item)
    }
}

impl Signable for TransactionLegacy {
    fn get_bytes(&self, for_signing: bool) -> BytesMut {
        let mut rlp = RlpStream::new();
        self.encode_rlp(&mut rlp, for_signing);

        let mut buf: BytesMut = BytesMut::new();
        buf.extend_from_slice(rlp.out().as_ref());

        buf
    }

    fn encode_rlp(&self, rlp: &mut RlpStream, for_signing: bool) {
        rlp.begin_unbounded_list();

        rlp.append(&self.nonce);
        rlp.append(&self.gas_price);
        rlp.append(&self.gas_limit);
        rlp.append(&self.to);
        rlp.append(&self.value);
        rlp.append(&self.data);

        if !for_signing && self.sign.is_some() {
            if let Some(sign) = self.sign.as_ref() {
                rlp.append(&sign.v);
                rlp.append(&sign.r);
                rlp.append(&sign.s);
            }
        } else {
            rlp.append(&self.chain_id);
            rlp.append(&"");
            rlp.append(&"");
        }

        rlp.finalize_unbounded_list();
    }
}

#[cfg(test)]
mod test {
    use std::error::Error;

    use crate::types::transaction::Transaction;

    #[test]
    fn decode_legacy_no_signature_transaction() -> Result<(), Box<dyn Error>> {
        let data = "e10182271082271094e94f1fa4f27d9d288ffea234bb62e1fbc086ca0c8080018080";
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
    fn decode_legacy_transaction() -> Result<(), Box<dyn Error>> {
        let data =  "f86e8302511e85036e1d083a826b6c948f2d10257ebf6386426456de1b1792b507426548875319b3e6ceb7bf8025a06716fc3c5bebebe88e61bc25714647b262904f7c99bd69c25541c7a796a9727fa071908b9fc3ce08f164cf1844ce43864a9347b7820a8921eef7aa67c55399e0be";
        let data = hex::decode(data)?;

        let tx = Transaction::decode(&data)?;
        match tx {
            Transaction::Legacy(x) => {
                assert_eq!(x.chain_id, 1);
                match x.sign {
                    Some(sig) => {
                        assert_eq!(
                            "0x690b9a9e9aa1c9db991c7721a92d351db4fac990",
                            format!("{}", sig.from.unwrap())
                        );
                        assert_eq!(
                            "0xd103e725e13c9886eb787517e47647010d077b51bc3a0a8b7ae7fc5a9cf351e2",
                            format!("{}", sig.hash)
                        );
                        Ok(())
                    }
                    None => panic!("Missing signature"),
                }
            }
            _ => panic!("Wrong transaction type"),
        }
    }

    #[test]
    fn encode_legacy_transaction() -> Result<(), Box<dyn Error>> {
        let data_hex =  "f86e8302511e85036e1d083a826b6c948f2d10257ebf6386426456de1b1792b507426548875319b3e6ceb7bf8025a06716fc3c5bebebe88e61bc25714647b262904f7c99bd69c25541c7a796a9727fa071908b9fc3ce08f164cf1844ce43864a9347b7820a8921eef7aa67c55399e0be";
        let data = hex::decode(data_hex)?;

        let tx = Transaction::decode(&data)?;

        let encoded = tx.encode(false);
        let encoded_hex = hex::encode(&encoded);

        assert_eq!(data, encoded);
        assert_eq!(data_hex, &encoded_hex);

        Ok(())
    }
}
