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
pub struct Transaction1559 {
    pub chain_id: u64,
    pub nonce: U256,
    pub max_priority_fee_per_gas: U256,
    pub gas_limit: U256,
    pub max_fee_per_gas: U256,
    pub to: Address,
    pub value: U256,
    pub data: Vec<u8>,
    pub access_list: Vec<AccessList>,
    pub sign: Option<Signature>,
}

impl Decodable for Transaction1559 {
    fn decode(rlp: &rlp::Rlp) -> Result<Self, rlp::DecoderError> {
        let item_count = rlp.item_count()?;
        if item_count != 9 && item_count != 12 {
            return Err(rlp::DecoderError::Custom(
                "Invalid parameters for 1559 transaction",
            ));
        }

        let chain_id: u64 = rlp.val_at(0)?;
        let nonce: U256 = rlp.val_at(1)?;

        let max_priority_fee_per_gas: U256 = rlp.val_at(2)?;
        let max_fee_per_gas: U256 = rlp.val_at(3)?;
        let gas_limit: U256 = rlp.val_at(4)?;

        let to = rlp.val_at(5)?;

        let value: U256 = rlp.val_at(6)?;
        let data: Vec<u8> = rlp.val_at(7)?;
        let access_list = rlp.list_at(8)?;

        let mut item = Self {
            chain_id,
            nonce,
            max_priority_fee_per_gas,
            max_fee_per_gas,
            gas_limit,
            to,
            value,
            data,
            access_list,
            sign: None,
        };

        if item_count == 12 {
            let mut buf = BytesMut::new();
            buf.extend_from_slice(&[2]);
            buf.extend_from_slice(rlp.as_raw());

            let signature = Signature::create(&item, rlp, &buf, 9)
                .map_err(|_| rlp::DecoderError::Custom("Error while recovering signature"))?;

            item.sign = Some(signature);
        }

        Ok(item)
    }
}

impl Signable for Transaction1559 {
    fn get_bytes(&self, for_signing: bool) -> bytes::BytesMut {
        let mut rlp = RlpStream::new();
        self.encode_rlp(&mut rlp, for_signing);

        let mut buf: BytesMut = BytesMut::new();
        buf.put_u8(2u8); //write EIP2930 identifier
        buf.extend_from_slice(rlp.out().as_ref());

        buf
    }

    fn encode_rlp(&self, rlp: &mut RlpStream, for_signing: bool) {
        rlp.begin_unbounded_list();

        rlp.append(&self.chain_id);
        rlp.append(&self.nonce);
        rlp.append(&self.max_priority_fee_per_gas);
        rlp.append(&self.max_fee_per_gas);
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
    fn decode_1559_transaction() -> Result<(), Box<dyn Error>> {
        let data =  "0x02f8b10108840f7f4900850519f0118083055845943fe65692bfcd0e6cf84cb1e7d24108e434a7587e80b8447050ccd90000000000000000000000005b1578681d43931030fffe066a072133842dde430000000000000000000000000000000000000000000000000000000000000001c001a0cac74d8e73874331e4ba78809a7b30574c76d35c43ea00983c76752b65a0632aa04b6caeac4c693f082b8bd4d2ad349848bc71b9d32a419e33a5a303a3b3f4e8c8";
        let data = hex::decode(data.trim_start_matches("0x"))?;

        let tx = Transaction::decode(&data)?;

        match tx {
            Transaction::EIP1559(x) => match x.sign {
                Some(sig) => {
                    assert_eq!(
                        "0x5b1578681d43931030fffe066a072133842dde43",
                        format!("{}", sig.from.unwrap())
                    );
                    assert_eq!(
                        "0x89f35f37f590d0f22b5ab8287bcd2d8a47683ef282b6ad4b2552ab01931faf36",
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
    fn encode_1559_transaction() -> Result<(), Box<dyn Error>> {
        let data_hex =  "0x02f8b10108840f7f4900850519f0118083055845943fe65692bfcd0e6cf84cb1e7d24108e434a7587e80b8447050ccd90000000000000000000000005b1578681d43931030fffe066a072133842dde430000000000000000000000000000000000000000000000000000000000000001c001a0cac74d8e73874331e4ba78809a7b30574c76d35c43ea00983c76752b65a0632aa04b6caeac4c693f082b8bd4d2ad349848bc71b9d32a419e33a5a303a3b3f4e8c8";
        let data = hex::decode(data_hex.trim_start_matches("0x"))?;

        let tx = Transaction::decode(&data)?;
        let encoded = tx.encode(false).to_vec();
        let encoded_hex = format!("0x{}", hex::encode(&encoded));

        assert_eq!(data, encoded);
        assert_eq!(data_hex, &encoded_hex);

        Ok(())
    }

    #[test]
    fn encode_1559_access_list_transaction() -> Result<(), Box<dyn Error>> {
        let data_hex =  "0x02f9051901820a66844d502d158503ee8a737083035d6f94000000001d68ffe32f650281ff45ffcb93b055a580b8ab54abfa6b839246c4804ca91538ac72cd41bcf05424e6f19dab7d43317344282f803f8e8d240708174a0000000002039ae5e27cddc6ac7b00634ed9079d3ad33d538c3ef57a0753fb16a59390afaf5dff4b2ce30a945708fd2481000000000000041e05d2f4491ba6cdf7028ceab81fa0c6971208e83fa7872994bee58167a1117691f39e05e9131cfa88f0e3a620e96700020000000000000000038c22000000000000042a35b8ac8ec191f903fdf89b94c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2f884a02cacd6ecd88e67356ddc78a780214f12b7a52f32434e36086bfdd8bbcfb7d14ba0c1de08830db2749ad8959eb14dca5d9786ce9ea39984eacff65e28a820266c26a01cdd5a182385e6cc1498df081ca576bd53a4b19dda3766a3406201a11cf7df65a03a0f132b80eb284c2ac247698d2ec7dce851ca420b700161ad6082d0e9762150d6940000000000000000000000000000000000000000c0f8dd94e6f19dab7d43317344282f803f8e8d240708174af8c6a0000000000000000000000000000000000000000000000000000000000000000ca00000000000000000000000000000000000000000000000000000000000000008a00000000000000000000000000000000000000000000000000000000000000006a00000000000000000000000000000000000000000000000000000000000000007a00000000000000000000000000000000000000000000000000000000000000009a0000000000000000000000000000000000000000000000000000000000000000af8bc94abfa6b839246c4804ca91538ac72cd41bcf05424f8a5a00000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000004a00000000000000000000000000000000000000000000000000000000000000002a054cdd369e4e8a8515e52ca72ec816c2101831ad1f18bf44102ed171459c9b4f8a00000000000000000000000000000000000000000000000000000000000000008f8599485eee30c52b0b379b046fb0f85f4f3dc3009afecf842a0c352fe3b9a5f3808969fc7a7de264ff5d985b2a163a73a6abee6ab1e1d15c9b9a0f219792bec970abebde2df1ee9a3fe8857e90e72c7b96368ba32b5d95d5cfd08f89b94cdf7028ceab81fa0c6971208e83fa7872994bee5f884a08b2ec926c2eb426b31c4aa21d946e23b854538f661a329c166b20dcfe4e30ec6a000c8e0a349c9e5bcfca3764b4866e1064734ff0916b8421d0343452c1701ad9da09317159d45a07e1cbf5e4dee1b8045dec3a0b69d5d8c1e5d0437a196007fa571a044be98e9a38b847ef1a5267a6675b71529ec05a942cc75553470b282e3812623f87a94ba12222222228d8ba445958a75a0704d566bf2c8f863a00000000000000000000000000000000000000000000000000000000000000000a09f33ba43609b1caef7a94c0433db35ceb04b7bf53a0fdacaee330a0749cce9aea09f33ba43609b1caef7a94c0433db35ceb04b7bf53a0fdacaee330a0749cce9aff7948167a1117691f39e05e9131cfa88f0e3a620e967e1a0000000000000000000000000000000000000000000000000000000000000000701a0106c2b96f3d56f31d865dae7dca3f1136603fd3de16fa717f9d6b27dda0f8ca1a05cd96b6c47baa11e18acce098b35800ca6de511973721fc5f5c9db4cadcc79b0";
        let data = hex::decode(data_hex.trim_start_matches("0x"))?;

        let tx = Transaction::decode(&data)?;
        let encoded = tx.encode(false).to_vec();
        let encoded_hex = format!("0x{}", hex::encode(&encoded));

        assert_eq!(data_hex, &encoded_hex);
        assert_eq!(data, encoded);

        match tx {
            Transaction::EIP1559(x) => {
                assert_eq!(x.access_list.len(), 8);
                Ok(())
            }
            _ => panic!("Wrong transaction type"),
        }
    }
}
