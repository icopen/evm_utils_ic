use ic_cdk::export::candid::{CandidType, Deserialize};
use rlp::{Decodable, Encodable, RlpStream};

use super::{address::Address, num::U256};

#[derive(CandidType, Deserialize, Clone, PartialEq, Eq)]
pub struct AccessList {
    pub address: Address,
    pub storage_keys: Vec<U256>,
}

impl Decodable for AccessList {
    fn decode(rlp: &rlp::Rlp) -> Result<Self, rlp::DecoderError> {
        let item_count = rlp.item_count()?;
        if item_count != 2 {
            return Err(rlp::DecoderError::Custom(
                "Invalid parameters for Access List",
            ));
        }

        let address = rlp.val_at(0)?;
        let storage_keys = rlp.list_at(1)?;
        Ok(Self {
            address,
            storage_keys,
        })
    }
}

impl Encodable for AccessList {
    fn rlp_append(&self, rlp: &mut RlpStream) {
        rlp.begin_list(2);
        rlp.append(&self.address);
        rlp.append_list(&self.storage_keys);
    }
}
