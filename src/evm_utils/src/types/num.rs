use ic_cdk::export::candid::{CandidType, Deserialize};
use rlp::{Decodable, Encodable, RlpStream, DecoderError, Rlp};

#[derive(CandidType, Deserialize, Clone, PartialEq, Eq)]
pub struct U256(pub [u8; 32]);
impl U256 {
    pub fn zero() -> Self {
        Self([0u8;32])
    }
    pub fn leading_zeros(&self) -> usize {
        let mut count = 0;

        for val in self.0 {
            if val == 0 {
                count += 1;
            } else {
                break;
            }
        }

        count
    }

    pub fn is_zero(&self) -> bool {
        for val in self.0 {
            if val != 0 {
                return false;
            }
        }

        true
    }

    #[inline]
    pub fn as_u64(&self) -> u64 {
        let mut buf = [0u8; 8];
        buf.copy_from_slice(&self.0[24..32]);
        u64::from_be_bytes(buf)
    }

    // pub fn as_u64(&self) -> u64 {

    // }

    // Whether this fits u64.
    #[inline]
    fn fits_word(&self) -> bool {
        let U256(ref arr) = self;
        for i in 1..24 { if arr[i] != 0 { return false; } }
        return true;
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

impl Encodable for U256 {
    fn rlp_append(&self, s: &mut RlpStream) {
        let leading_empty_bytes = self.leading_zeros() as usize / 8;
        let buffer = self.0;

        s.encoder().encode_value(&buffer[leading_empty_bytes..]);
    }
}

impl Decodable for U256 {
    fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
        rlp.decoder().decode_value(|bytes| match bytes.len() {
            0 => Ok(U256::zero()),
            l if l <= 32 => {
                if bytes[0] == 0 {
                    return Err(DecoderError::RlpInvalidIndirection)
                }
                let mut res = U256::zero();

                for (i, byte) in bytes.iter().enumerate().take(l) {
                    res.0[32 - l+i] = *byte;
                }
                Ok(res)
            },
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
    fn to_u64() {
        let mut buf = [0u8; 32];
        buf[30] = 4;

        let converted = U256(buf);
        let num = converted.as_u64();

        assert_eq!(1024, num);
    }

    #[test]
    fn is_zero() {
        let item = U256::zero();
        assert!(item.is_zero());

        let item2 = U256([1u8; 32]);
        assert!(!item2.is_zero());
    }
}