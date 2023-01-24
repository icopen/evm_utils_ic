use ic_cdk::export::candid::{CandidType, Deserialize};
use rlp::{Decodable, Encodable, RlpStream};

#[derive(CandidType, Deserialize, Clone, PartialEq, Eq)]
pub struct U256(pub [u8; 32]);
impl From<[u8; 32]> for U256 {
    #[inline]
    fn from(bytes: [u8; 32]) -> Self {
        U256(bytes)
    }
}

impl<'a> From<&'a [u8; 32]> for U256 {
    #[inline]
    fn from(bytes: &'a [u8; 32]) -> Self {
        U256(*bytes)
    }
}

impl Decodable for U256 {
    fn decode(rlp: &rlp::Rlp) -> Result<Self, rlp::DecoderError> {
        let raw: Vec<u8> = rlp.as_val()?;

        let mut data = [0_u8; 32];

        if raw.len() > 32 {
            Err(rlp::DecoderError::Custom("Too much data for U256"))
        } else {
            // data.copy_from_slice(&raw);

            data[0] = raw[0];

            Ok(Self::from(&data))
        }
    }
}

impl Encodable for U256 {
    fn rlp_append(&self, rlp: &mut RlpStream) {
        // rlp.append_list(&self.values);
    }
}
