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
    let tx = ethers.utils.parseTransaction(encoded.Ok[0]); //First value is tx

    let hash = ethers.utils.hexlify(encoded.Ok[1]);

    let signing_tx = {
        chainId: tx.chainId,
        data: tx.data,
        to: tx.to,
        gasLimit: tx.gasLimit,
        gasPrice: tx.gasPrice,
        nonce: tx.nonce,
        type: tx.type,
        value: tx.value
    };

    const raw = ethers.utils.serializeTransaction(signing_tx); // returns RLP encoded tx
    const msgHash = ethers.utils.keccak256(raw);

    expect(tx.to.toLocaleLowerCase()).toBe("0xe94f1fa4f27d9d288ffea234bb62e1fbc086ca0c");
    expect(tx.chainId).toStrictEqual(1);
    expect(tx.nonce).toStrictEqual(1);
    expect(tx.gasLimit).toStrictEqual(BigNumber.from(10_000));
    expect(tx.gasPrice).toStrictEqual(BigNumber.from(10_000));

    expect(msgHash).toBe(hash);
});


test("parse_transaction", async () => {
    let ser = {
            to: "0xe94f1fa4f27d9d288ffea234bb62e1fbc086ca0c",
            value: 0,
            data: [],
            chainId: 1,
            nonce: 1,
            gasLimit: 10_000,
            gasPrice: 10_000
    };

    let raw_tx = ethers.utils.serializeTransaction(ser);

    console.log(raw_tx);

    let decoded = await can.parse_transaction(ethers.utils.arrayify(raw_tx));

    expect(decoded.Ok.Legacy).not.toBeUndefined();
    expect(decoded.Ok.Legacy.gas_limit).toBe(10_000n);
    expect(decoded.Ok.Legacy.gas_price).toBe(10_000n);
    expect(decoded.Ok.Legacy.nonce).toBe(1n);
    expect(decoded.Ok.Legacy.chain_id).toBe(1n);
});


