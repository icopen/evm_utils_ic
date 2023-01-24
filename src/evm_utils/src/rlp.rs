use crate::types::rlp::List;

use ic_cdk_macros::query;

#[query]
fn rlp_encode(data: List) -> Result<Vec<u8>, String> {
    let raw = rlp::encode(&data);

    Ok(raw.to_vec())
}

#[query]
fn rlp_decode(raw: Vec<u8>) -> Result<List, String> {
    let item: List = rlp::decode(&raw).map_err(|x| format!("{}", x))?;

    Ok(item)
}

#[cfg(test)]
mod test {
    use crate::{
        rlp::{rlp_decode, rlp_encode},
        types::rlp::List,
    };

    #[test]
    fn encode_decode_test() -> Result<(), String> {
        let item = List { values: vec![] };

        let encoded = rlp_encode(item.clone())?;

        let decoded = rlp_decode(encoded)?;

        assert_eq!(item.values.len(), decoded.values.len());

        Ok(())
    }
}
