use candid::candid_method;
use hasher::HasherKeccak;
use ic_cdk::query;

#[query]
#[candid_method(query)]
fn verify_proof(
    root: Vec<u8>,
    key: Vec<u8>,
    proof: Vec<Vec<u8>>,
) -> Result<Option<Vec<u8>>, String> {
    let hasher = HasherKeccak::new();

    let data = cita_trie::verify_proof(&root, &key, proof, hasher)
        .map_err(|x| format!("Error while verifying proof {x}"))?;

    Ok(data)
}
