import { getActor } from "./_common";
import { BigNumber, ethers } from "ethers";
import { expect } from "@jest/globals";


const can = await getActor(false, "evm_utils");

test("create_transaction", async () => {
    let item = {
        Legacy: {
            to: ethers.utils.arrayify(ethers.utils.hexlify("0xe94f1fa4f27d9d288ffea234bb62e1fbc086ca0c")),
            value: 0,
            data: [],
            sign: [],
            chain_id: 1,
            nonce: 1,
            gas_limit: 10_000,
            gas_price: 10_000
        }
    };

    let encoded = await can.create_transaction(item);
    console.log(encoded);

    let tx = ethers.utils.parseTransaction(encoded.Ok);
    console.log(tx);

    expect(tx.to.toLocaleLowerCase()).toBe("0xe94f1fa4f27d9d288ffea234bb62e1fbc086ca0c");
    expect(tx.chainId).toStrictEqual(1);
    expect(tx.nonce).toStrictEqual(1);
    expect(tx.gasLimit).toStrictEqual(BigNumber.from(10_000));
    expect(tx.gasPrice).toStrictEqual(BigNumber.from(10_000));
});

// test("parse_transaction", async () => {
//     let bytes = ethers.utils.arrayify(ethers.utils.hexlify("0xc9845445737481e63201"));
    
//     let decoded = await can.rlp_decode(bytes);
//     expect(decoded.Ok.values.length).toBe(4);
// });


