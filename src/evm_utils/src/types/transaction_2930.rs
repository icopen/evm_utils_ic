use bytes::{BufMut, BytesMut};
use ic_cdk::export::candid::{CandidType, Deserialize};
use rlp::{Decodable, RlpStream};

use super::{
    access_list::AccessList,
    address::Address,
    num::U256,
    signature::{Signable, Signature},
};

#[derive(CandidType, Deserialize, Clone, PartialEq, Eq)]
pub struct Transaction2930 {
    pub chain_id: u64,
    pub nonce: U256,
    pub gas_price: U256,
    pub gas_limit: U256,
    pub to: Address,
    pub value: U256,
    pub data: Vec<u8>,
    pub access_list: Vec<AccessList>,
    pub sign: Option<Signature>,
}

impl Decodable for Transaction2930 {
    fn decode(rlp: &rlp::Rlp) -> Result<Self, rlp::DecoderError> {
        let item_count = rlp.item_count()?;
        if item_count != 8 && item_count != 11 {
            return Err(rlp::DecoderError::Custom(
                "Invalid parameters for 2930 transaction",
            ));
        }

        let chain_id: u64 = rlp.val_at(0)?;
        let nonce: U256 = rlp.val_at(1)?;

        let gas_price: U256 = rlp.val_at(2)?;
        let gas_limit: U256 = rlp.val_at(3)?;

        let to = rlp.val_at(4)?;

        let value: U256 = rlp.val_at(5)?;
        let data: Vec<u8> = rlp.val_at(6)?;
        let access_list = rlp.list_at(7)?;

        let mut item = Self {
            chain_id,
            nonce,
            gas_price,
            gas_limit,
            to,
            value,
            data,
            access_list,
            sign: None,
        };

        if item_count == 11 {
            let mut buf = BytesMut::new();
            buf.extend_from_slice(&[1]);
            buf.extend_from_slice(rlp.as_raw());

            let signature = Signature::create(&item, rlp, &buf, 8)
                .map_err(|_| rlp::DecoderError::Custom("Error while recovering signature"))?;

            item.sign = Some(signature);
        }

        Ok(item)
    }
}

impl Signable for Transaction2930 {
    fn get_bytes(&self, for_signing: bool) -> bytes::BytesMut {
        let mut rlp = RlpStream::new();
        self.encode_rlp(&mut rlp, for_signing);

        let mut buf: BytesMut = BytesMut::new();
        buf.put_u8(1u8); //write EIP2930 identifier
        buf.extend_from_slice(rlp.out().as_ref());

        buf
    }

    fn encode_rlp(&self, rlp: &mut RlpStream, for_signing: bool) {
        rlp.begin_unbounded_list();

        rlp.append(&self.chain_id);
        rlp.append(&self.nonce);
        rlp.append(&self.gas_price);
        rlp.append(&self.gas_limit);
        rlp.append(&self.to);
        rlp.append(&self.value);
        rlp.append(&self.data);
        rlp.append_list(&self.access_list);

        if !for_signing && self.sign.is_some() {
            if let Some(sign) = self.sign.as_ref() {
                rlp.append(&sign.v);
                rlp.append(&sign.r);
                rlp.append(&sign.s);
            }
        }

        rlp.finalize_unbounded_list();
    }
}

#[cfg(test)]
mod test {
    use crate::types::transaction::Transaction;
    use std::error::Error;

    #[test]
    fn decode_2930_transaction() -> Result<(), Box<dyn Error>> {
        let data =  "0x01f8ab010485032af8977482d91194bbbbca6a901c926f240b89eacb641d8aec7aeafd80b844095ea7b3000000000000000000000000000000000022d473030f116ddee9f6b43ac78ba3ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc001a039c31c2e8693b56c4dc7b07fc77e1bfa6dd74f0cf55450f15a3c3e8ce0e2540ca071b92c8319f7415ba131e07f5481a20388bfd234640b58ae46c94ab2a5c2b7ea";
        let data = hex::decode(data.trim_start_matches("0x"))?;

        let tx = Transaction::decode(&data)?;

        match tx {
            Transaction::EIP2930(x) => match x.sign {
                Some(sig) => {
                    assert_eq!(
                        "0xef9ae1e5329f145dfbc5a4c435601a1a22a41fd0",
                        format!("{}", sig.from.unwrap())
                    );
                    assert_eq!(
                        "0x50f38b90c94d32726648b36cacfafbf71a07e898fe1d1dbc692aa751cbf296cd",
                        format!("{}", sig.hash)
                    );
                    Ok(())
                }
                None => panic!("Missing signature"),
            },
            _ => panic!("Wrong transaction type"),
        }
    }

    #[test]
    fn encode_2930_transaction() -> Result<(), Box<dyn Error>> {
        let data_hex =  "0x01f8ab010485032af8977482d91194bbbbca6a901c926f240b89eacb641d8aec7aeafd80b844095ea7b3000000000000000000000000000000000022d473030f116ddee9f6b43ac78ba3ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc001a039c31c2e8693b56c4dc7b07fc77e1bfa6dd74f0cf55450f15a3c3e8ce0e2540ca071b92c8319f7415ba131e07f5481a20388bfd234640b58ae46c94ab2a5c2b7ea";
        let data = hex::decode(data_hex.trim_start_matches("0x"))?;

        let tx = Transaction::decode(&data)?;

        let encoded = tx.encode(false);
        let encoded_hex = format!("0x{}", hex::encode(&encoded));

        assert_eq!(data, encoded);
        assert_eq!(data_hex, &encoded_hex);

        Ok(())
    }

    #[test]
    fn encode_2930_access_list_transaction() -> Result<(), Box<dyn Error>> {
        let data_hex =  "0x01f906f701824c2085144790bc4f8304bdc394b9aa14b6774dbc9f84b15e7ff659c0dba0d9ddf5843b9aca00b8612e0000000000a0d4c2e5955530774b6a5dafaa45576c3938d779f9d7285aa45336bd2f2c9b79c7ddad45ed81791ce9abe0a0205b1e2773caa898ad7043ec4df2d22526ed331d54b6710000000000fc0f617ce7bc1300000011c497bae6763c10c2f90626f8dd94bd2f2c9b79c7ddad45ed81791ce9abe0a0205b1ef8c6a00000000000000000000000000000000000000000000000000000000000000102a000000000000000000000000000000000000000000000000000000000000000cca00000000000000000000000000000000000000000000000000000000000000100a000000000000000000000000000000000000000000000000000000000000000fea000000000000000000000000000000000000000000000000000000000000000ffa00000000000000000000000000000000000000000000000000000000000000101d6944a4efa663f98080227b0c8613976f985f2525d60c0f9026a942773caa898ad7043ec4df2d22526ed331d54b671f90252a0486cd66ca5810865f2eaa5f4fbb5d395a8e6d7e9dbebb5efbcb4bf93a23eb8f3a0d0fcde32b0215526075114b6a6af4706af88a4a1be3fa34388e6e5ac86c20d26a00000000000000000000000000000000000000000000000000000000000000008a0b1e5e7ab4ee56223f64c81cb8b785eb767023d97e06dac8879cf900afb990219a0d0fcde32b0215526075114b6a6af4706af88a4a1be3fa34388e6e5ac86c20d25a09c04773acff4c5c42718bd0120c72761f458e43068a3961eb935577d1ed4effba0b1e5e7ab4ee56223f64c81cb8b785eb767023d97e06dac8879cf900afb99021aa0b1e5e7ab4ee56223f64c81cb8b785eb767023d97e06dac8879cf900afb99021ba0b1e5e7ab4ee56223f64c81cb8b785eb767023d97e06dac8879cf900afb990218a0486cd66ca5810865f2eaa5f4fbb5d395a8e6d7e9dbebb5efbcb4bf93a23eb8f1a0486cd66ca5810865f2eaa5f4fbb5d395a8e6d7e9dbebb5efbcb4bf93a23eb8f2a0d0fcde32b0215526075114b6a6af4706af88a4a1be3fa34388e6e5ac86c20d24a00000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000004a0486cd66ca5810865f2eaa5f4fbb5d395a8e6d7e9dbebb5efbcb4bf93a23eb8f0a0d0fcde32b0215526075114b6a6af4706af88a4a1be3fa34388e6e5ac86c20d23a00000000000000000000000000000000000000000000000000000000000000001a00000000000000000000000000000000000000000000000000000000000000002f87a94c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2f863a0063467257e2af4c74a25e3e592134cf6a6796ea52a56b05c49fdbc0ea832b1dca0cc70ab56ed339e7c9e9bf51bcf50d1d180b2b10e715a53b3c643e78e6828d624a033df01d1414964c290c14fc9dc9e4ff05f7239b18ee44e6ddfbad504a04bdeb1f79423b0c2381075df4002bc6d3b9baf52ab0acb1e9be1a00000000000000000000000000000000000000000000000000000000000000001d69488c3e2ac77fcd790ffc2cbb0f10f20776851e2e2c0f8bc94774b6a5dafaa45576c3938d779f9d7285aa45336f8a5a00000000000000000000000000000000000000000000000000000000000000004a00000000000000000000000000000000000000000000000000000000000000002a09c04773acff4c5c42718bd0120c72761f458e43068a3961eb935577d1ed4effba00000000000000000000000000000000000000000000000000000000000000008a00000000000000000000000000000000000000000000000000000000000000000f89b94881ba05de1e78f549cc63a8f6cabb1d4ad32250df884a0b55088d21d7a3d68b4dd0838035e491cf9632e8cc016506869e8803cb6fb31c6a00c23f089f2f76e2e5adec531a861a1a11cd50e704a930566c29856c414f10e25a0f1dda771943dc44f5c3a03fb3b53948058cc3c10d63d55a42fa0f2e7850ad18da0b4c0f9e4fcbeeab1458e94fe8faca61c369cde841e2dfbdb8914336724a9a639f89b947476d8b314607990957dda4479acf44ffa552034f884a0455ea71e93b0b19f9ed513d6f2b2a7c0a97189bb529df422b82a14b50eab6e84a00c23f089f2f76e2e5adec531a861a1a11cd50e704a930566c29856c414f10e25a00a867bf58e9a359b89c036573f9d6987985d133d090037fbb06c6d98683c45cea0f1dda771943dc44f5c3a03fb3b53948058cc3c10d63d55a42fa0f2e7850ad18d80a0c0fc871e2da1b57151046b77094bfc95cb953898965425a2f9acf8ac6f85d700a04589d36c9c07e7dbb5b494cd336d4d389be2a1689bc6f91a4a97db1ad5095d38";
        let data = hex::decode(data_hex.trim_start_matches("0x"))?;

        let tx = Transaction::decode(&data)?;
        let encoded = tx.encode(false).to_vec();
        let encoded_hex = format!("0x{}", hex::encode(&encoded));

        assert_eq!(data, encoded);
        assert_eq!(data_hex, &encoded_hex);

        match tx {
            Transaction::EIP2930(x) => {
                assert_eq!(x.access_list.len(), 9);
                Ok(())
            }
            _ => panic!("Wrong transaction type"),
        }
    }
}
