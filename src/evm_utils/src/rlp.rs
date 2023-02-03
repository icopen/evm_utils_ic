use crate::types::rlp::List;

use candid::candid_method;
use ic_cdk_macros::query;

#[query]
#[candid_method(query)]
fn rlp_encode(data: List) -> Result<Vec<u8>, String> {
    let raw = rlp::encode(&data);

    Ok(raw.to_vec())
}

#[query]
#[candid_method(query)]
fn rlp_decode(raw: Vec<u8>) -> Result<List, String> {
    let item: List = rlp::decode(&raw).map_err(|x| format!("{x}"))?;

    Ok(item)
}

#[cfg(test)]
mod test {
    use crate::{
        rlp::{rlp_decode, rlp_encode},
        types::rlp::{Item, List},
    };

    #[test]
    fn encode_decode_test() -> Result<(), String> {
        let item = List {
            values: vec![Item::Text(String::from("test")), Item::Num(64), Item::Empty],
        };

        let encoded = rlp_encode(item.clone())?;

        let hex_encoded = hex::encode(&encoded);
        println!("{hex_encoded}");

        let decoded = rlp_decode(encoded)?;

        assert_eq!(item.values.len(), decoded.values.len());

        Ok(())
    }
}
