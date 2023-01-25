use std::{fmt::Display};

use ic_cdk::export::candid::{CandidType, Deserialize};
use rlp::{Decodable, Encodable, RlpStream, DecoderError, Rlp};

#[derive(CandidType, Deserialize, Clone, PartialEq, Eq)]
pub struct Address(pub [u8; 20]);

impl Decodable for Address {
    fn decode(rlp: &rlp::Rlp) -> Result<Self, rlp::DecoderError> {
        let data = rlp.data()?;
        if data.len() != 20 {
            return Err(DecoderError::Custom("Invalid number of bytes for ETH address"));
        }

        let mut buf = [0u8;20];
        buf.copy_from_slice(&data[..20]);
        Ok(Self(buf))
    }
}

impl Encodable for Address {
    fn rlp_append(&self, rlp: &mut RlpStream) {
        rlp.append_internal(&self.0.to_vec());
    }
}

impl Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x")?;
        for byte in self.0 {
            write!(f, "{:x}", byte)?;
        }
        Ok(())
    }
}
