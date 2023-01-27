use std::fmt::Display;

use ic_cdk::export::candid::{CandidType, Deserialize};
use rlp::{Decodable, DecoderError, Encodable, Rlp, RlpStream};
use secp256k1::PublicKey;

use crate::utils::keccak256;

#[derive(CandidType, Deserialize, Clone, PartialEq, Eq)]
pub struct Address(pub [u8; 20]);

impl Decodable for Address {
    fn decode(rlp: &rlp::Rlp) -> Result<Self, rlp::DecoderError> {
        let data = rlp.data()?;
        if data.len() != 20 {
            return Err(DecoderError::Custom(
                "Invalid number of bytes for ETH address",
            ));
        }

        let mut buf = [0u8; 20];
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
        write!(f, "{}", hex::encode(self.0))?;
        Ok(())
    }
}

impl From<PublicKey> for Address {
    fn from(from: PublicKey) -> Self {
        let addr = &keccak256(&[&from.serialize_uncompressed()[1..]]).0[12..];
        let mut buf = [0u8; 20];
        buf.copy_from_slice(&addr[..20]);

        Self(buf)
    }
}

//todo: add tests for conversion from public key to address!
#[cfg(test)]
mod test {
    use std::str::FromStr;
    use std::error::Error;

    use secp256k1::PublicKey;

    use super::Address;

    #[test]
    fn test_public_key_to_address() -> Result<(), Box<dyn Error>> {
        let key = PublicKey::from_str("04da2bd30515dc22663fa0fd96e3a866b6858876f83d990953065cacf1fa6de3e441e85d533c3cb7f7fb94a73dbb01c22b9340ef4a9940c6a81adea958effe0c8a")?;
        let addr = Address::from(key);

        let addr_str = format!("{}", addr);

        assert_eq!(addr_str, "0x690b9a9e9aa1c9db991c7721a92d351db4fac990");

        Ok(())
    }
}