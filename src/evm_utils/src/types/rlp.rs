use ic_cdk::export::candid::{CandidType, Deserialize};
use rlp::{Decodable, Encodable, RlpStream};

#[derive(CandidType, Deserialize, Clone)]
pub enum Item {
    Text(String),
    Num(u64),
    List(List),
    Raw(Vec<u8>),
    Empty,
}

impl Decodable for Item {
    fn decode(rlp: &rlp::Rlp) -> Result<Self, rlp::DecoderError> {
        let mut item = Item::Empty;

        if rlp.is_list() {
            let i: List = rlp.as_val()?;
            item = Item::List(i);
        }
        if rlp.is_int() {
            let i: u64 = rlp.as_val()?;
            item = Item::Num(i);
        }
        if rlp.is_data() {
            let i: Vec<u8> = rlp.as_val()?;
            item = Item::Raw(i);
        }

        Ok(item)
    }
}

impl Encodable for Item {
    fn rlp_append(&self, rlp: &mut RlpStream) {
        // rlp.append_list(&self.values);
    }
}

#[derive(CandidType, Deserialize, Clone)]
pub struct List {
    pub values: Vec<Item>,
}

impl Decodable for List {
    fn decode(rlp: &rlp::Rlp) -> Result<Self, rlp::DecoderError> {
        // let name: String = rlp.val_at(0)?;
        // let ver: usize = rlp.val_at(1)?;

        if !rlp.is_list() {
            return Err(rlp::DecoderError::RlpExpectedToBeList);
        }

        let mut item = Self { values: vec![] };

        for i in rlp.into_iter() {
            let data = i.as_val::<Item>()?;
            item.values.push(data);
        }

        Ok(item)
    }
}

impl Encodable for List {
    fn rlp_append(&self, rlp: &mut RlpStream) {
        rlp.append_list(&self.values);
    }
}
