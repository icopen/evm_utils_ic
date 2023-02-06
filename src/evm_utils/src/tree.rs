use candid::candid_method;
use hasher::HasherKeccak;
use ic_cdk::query;

use crate::utils::keccak256;

#[query]
#[candid_method(query)]
fn verify_proof(
    root: Vec<u8>,
    key: Vec<u8>,
    proof: Vec<Vec<u8>>,
) -> Result<Option<Vec<u8>>, String> {
    let hasher = HasherKeccak::new();
    let hashed_key = keccak256(&[&key]);

    let data = cita_trie::verify_proof(&root, &hashed_key.0, proof, hasher)
        .map_err(|x| format!("Error while verifying proof {x}"))?;

    Ok(data)
}

#[cfg(test)]
mod test {
    use crate::{types::num::U256, utils::keccak256};
    use hasher::HasherKeccak;
    use std::error::Error;
    use std::fmt::{self, Display};

    #[derive(Debug)]
    pub enum TestError {
        NotFound,
    }

    impl std::error::Error for TestError {}
    impl Display for TestError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str(&format!("{self:?}"))?;
            Ok(())
        }
    }

    #[test]
    fn verify_proof_test() -> Result<(), Box<dyn Error>> {
        let root = hex::decode("7317ebbe7d6c43dd6944ed0e2c5f79762113cb75fa0bed7124377c0814737fb4")?;
        let proof = vec![
            hex::decode("f8518080a0cd2a98a2ebb71b70e1109bf206dbc96dc73c76569b42df09ff269ecdcd31b1398080808080808080a0236e8f61ecde6abfebc6c529441f782f62469d8a2cc47b7aace2c136bd3b1ff08080808080")?,
            hex::decode("f7a0390decd9548b62a8d60345a988386fc84ba6bc95484008f6362f93160ef3e5639594de74da73d5102a796559933296c73e7d1c6f37fb")?,
        ];
        let hasher = HasherKeccak::new();

        let pos = U256::zero();
        let hashed_pos = keccak256(&[&pos.0]);

        let result = cita_trie::verify_proof(&root, &hashed_pos.0, proof, hasher)?;

        match result {
            Some(_) => Ok(()),
            None => Err(Box::new(TestError::NotFound)),
        }
    }
}
