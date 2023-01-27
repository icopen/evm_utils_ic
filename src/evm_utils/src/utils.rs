use std::error::Error;

use secp256k1::Message;
use secp256k1::PublicKey;
use secp256k1::ecdsa::RecoverableSignature;
use secp256k1::ecdsa::RecoveryId;
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

/// Recovers public key of a message signer
pub fn recover_sender(r: &[u8], s: &[u8], v: u64, msg: &[u8]) -> Result<PublicKey, Box<dyn Error>> {
    let mut sign = [0u8; 64];

    sign[..32].copy_from_slice(&r[..32]);
    sign[32..].copy_from_slice(&s[..32]);

    let mut rec_id = v;
    if rec_id > 1 {
        rec_id -= 37;
    }

    let rec_id = RecoveryId::from_i32(rec_id as i32)?;
    let rec_sig = RecoverableSignature::from_compact(&sign, rec_id)?;

    let hash = keccak256(&[&msg]);
    let msg = Message::from_slice(&hash.0)?;

    println!("r {}", hex::encode(r));
    println!("s {}", hex::encode(s));
    println!("v {}", v);
    println!("msg {}", msg);

    let pub_k = rec_sig.recover(&msg)?;
    Ok(pub_k)
}