use sha3::Digest;
use sha3::Keccak256;

use crate::types::num::U256;

/// Convenience function for calculation of keccak256 hash
pub fn keccak256(data: &[&[u8]]) -> U256 {
    let mut hasher = Keccak256::new();
    for i in data {
        hasher.update(i);
    }

    U256::from(hasher.finalize().as_ref())
}
