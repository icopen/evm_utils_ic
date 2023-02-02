use std::fmt::Display;

use ic_cdk::export::candid::{CandidType, Deserialize};
use rlp::{Decodable, DecoderError, Encodable, Rlp, RlpStream};

#[derive(CandidType, Deserialize, Clone, PartialEq, Eq)]
pub struct U256(pub [u8; 32]);
impl U256 {
    pub fn zero() -> Self {
        Self([0u8; 32])
    }
}

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

impl From<u64> for U256 {
    #[inline]
    fn from(num: u64) -> Self {
        let mut buf = [0u8; 32];
        buf[24..32].copy_from_slice(&num.to_be_bytes());
        U256(buf)
    }
}

impl Display for U256 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x")?;
        write!(f, "{}", hex::encode(self.0))
    }
}

impl Encodable for U256 {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.encoder().encode_value(&self.0);
    }
}

impl Decodable for U256 {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        rlp.decoder().decode_value(|bytes| match bytes.len() {
            0 => Ok(U256::zero()),
            l if l <= 32 => {
                let mut res = U256::zero();

                for (i, byte) in bytes.iter().enumerate().take(l) {
                    res.0[32 - l + i] = *byte;
                }
                Ok(res)
            }
            _ => Err(DecoderError::RlpIsTooBig),
        })
    }
}

#[cfg(test)]
mod test {
    use super::U256;

    #[test]
    fn from_u64() {
        let num = 1024u64;

        let converted = U256::from(num);
        assert_eq!(converted.0[30], 4);
    }

    #[test]
    fn rlp_encode_zero() {
        let num = U256::from(10u64);
        let encoded = rlp::encode(&num);

        let hex_data = hex::encode(&encoded);

        println!("{hex_data}");
    }
}
