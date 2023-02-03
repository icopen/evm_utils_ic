use std::error::Error;

use candid::candid_method;
use ic_cdk::query;
use secp256k1::ecdsa::RecoverableSignature;
use secp256k1::ecdsa::RecoveryId;
use secp256k1::Message;
use secp256k1::PublicKey;
use sha3::Digest;
use sha3::Keccak256;

use crate::types::address::Address;
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
pub fn _recover_public_key(
    r: &[u8],
    s: &[u8],
    v: u64,
    msg: &[u8],
) -> Result<PublicKey, Box<dyn Error>> {
    let mut sign = [0u8; 64];

    sign[..32].copy_from_slice(&r[..32]);
    sign[32..].copy_from_slice(&s[..32]);

    let mut rec_id = v;
    if rec_id > 1 {
        rec_id -= 37;
    }

    let rec_id = RecoveryId::from_i32(rec_id as i32)?;
    let rec_sig = RecoverableSignature::from_compact(&sign, rec_id)?;

    let hash = keccak256(&[msg]);
    let msg = Message::from_slice(&hash.0)?;

    let pub_k = rec_sig.recover(&msg)?;
    Ok(pub_k)
}

#[query]
#[candid_method(query)]
fn recover_public_key(signature: Vec<u8>, msg: Vec<u8>) -> Result<Vec<u8>, String> {
    if signature.len() != 65 {
        Err(String::from("Invalid signature length!, should be 65"))
    } else {
        let public_key = _recover_public_key(
            &signature[..32],
            &signature[32..64],
            signature[64] as u64,
            &msg,
        )
        .map_err(|x| format!("Error while recovering public key {x}"))?;

        Ok(public_key.serialize_uncompressed().to_vec())
    }
}

#[query]
#[candid_method(query)]
fn pub_to_address(public_key: Vec<u8>) -> Result<Vec<u8>, String> {
    let pub_k = PublicKey::from_slice(&public_key[..])
        .map_err(|x| format!("Error while reading public key {x}"))?;

    let addr = Address::from(pub_k);

    Ok(addr.0.to_vec())
}

#[query]
#[candid_method(query)]
fn is_valid_public(public_key: Vec<u8>) -> Result<(), String> {
    PublicKey::from_slice(&public_key[..])
        .map_err(|x| format!("Error while reading public key {x}"))?;

    Ok(())
}

#[query]
#[candid_method(query)]
fn is_valid_signature(signature: Vec<u8>) -> Result<(), String> {
    if signature.len() != 65 {
        Err(String::from("Invalid signature length!, should be 65"))
    } else {
        let mut rec_id = signature[64];
        if rec_id > 1 {
            rec_id -= 37;
        }
        let rec_id =
            RecoveryId::from_i32(rec_id as i32).map_err(|x| format!("Recovery Id error {x}"))?;

        RecoverableSignature::from_compact(&signature[..64], rec_id)
            .map_err(|x| format!("Recoverable signature error {x}"))?;

        Ok(())
    }
}
