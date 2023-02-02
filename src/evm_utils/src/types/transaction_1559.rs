use ic_cdk::export::candid::{CandidType, Deserialize};
use rlp::{Decodable, Encodable, RlpStream};

use super::address::Address;

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

        // let mut from = None;

        // let mut v = 0;
        // let mut r = vec![];
        // let mut s = vec![];

        // if item_count == 12 {
        //     v = rlp.val_at(9)?;
        //     r = rlp.val_at(10)?;
        //     s = rlp.val_at(11)?;

        //     let mut rlp = RlpStream::new();
        //     rlp.begin_list(9);
        //     rlp.append(&chain_id);
        //     rlp.append(&nonce);
        //     rlp.append(&max_priority_fee_per_gas);
        //     rlp.append(&max_fee_per_gas);
        //     rlp.append(&gas_limit);
        //     rlp.append(&to);
        //     rlp.append(&value);
        //     rlp.append(&data);
        //     rlp.append_raw(&access_list, access_list.len());

        //     let mut buf = BytesMut::new();
        //     buf.extend_from_slice(&[2]);
        //     buf.extend_from_slice(&rlp.out()[..]);

        //     let sender = _recover_public_key(&r, &s, v, &buf[..])
        //         .map_err(|_| rlp::DecoderError::Custom("Error decoding sender address"))?;

        //     from = Some(Address::from(sender));
        // }

        let item = Self {
            chain_id,
            nonce,
            max_priority_fee_per_gas,
            max_fee_per_gas,
            gas_limit,
            to,
            value,
            data,
            access_list,
        };

        Ok(item)
    }
}

impl Encodable for Transaction1559 {
    fn rlp_append(&self, rlp: &mut RlpStream) {
        rlp.begin_list(9);
        rlp.append(&self.chain_id);
        rlp.append(&self.nonce);
        rlp.append(&self.max_priority_fee_per_gas);
        rlp.append(&self.max_fee_per_gas);
        rlp.append(&self.gas_limit);
        rlp.append(&self.to);
        rlp.append(&self.value);
        rlp.append(&self.data);

        rlp.append_raw(&self.access_list, self.access_list.len());
    }
}
