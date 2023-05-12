import { getActor } from "./_common";
import { BigNumber, ethers } from "ethers";
import { expect } from "@jest/globals";


const can = await getActor();

test("create_transaction", async () => {
    let arr = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
    let ten_thousand_as_bytes = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 16];

    let item = {
        Legacy: {
            to: ethers.utils.arrayify(ethers.utils.hexlify("0xe94f1fa4f27d9d288ffea234bb62e1fbc086ca0c")),
            value: arr,
            data: [],
            sign: [],
            chain_id: 1,
            nonce: arr,
            gas_limit: ten_thousand_as_bytes,
            gas_price: ten_thousand_as_bytes
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

test("encode_signed_transaction", async () => {
    let arr = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
    let ten_thousand_as_bytes = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 16];

    let item = {
        Legacy: {
            to: ethers.utils.arrayify(ethers.utils.hexlify("0xe94f1fa4f27d9d288ffea234bb62e1fbc086ca0c")),
            value: arr,
            data: [],
            sign: [
                {
                    r: ethers.utils.arrayify(ethers.utils.hexlify("0xde8ef039a601ad81d0882568e503fd7d865d8e6e03c35f635b301263ce7e00c2")),
                    s: ethers.utils.arrayify(ethers.utils.hexlify("0x770ceb3615e121d5fa712de1d9b5b27c64dc7ee0610d19307210a6de4b347ad9")),
                    v: 38,
                    from: [],
                    hash: arr
                }
            ],
            chain_id: 1,
            nonce: arr,
            gas_limit: ten_thousand_as_bytes,
            gas_price: ten_thousand_as_bytes
        }
    };

    let encoded = await can.encode_signed_transaction(item);
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
    let signature_like = {
        r: ethers.utils.hexlify(item.Legacy.sign[0].r),
        s: ethers.utils.hexlify(item.Legacy.sign[0].s),
        v: item.Legacy.sign[0].v
    };

    const raw = ethers.utils.serializeTransaction(signing_tx, signature_like); // returns RLP encoded tx
    const msgHash = ethers.utils.keccak256(raw);

    expect(tx.to.toLocaleLowerCase()).toBe("0xe94f1fa4f27d9d288ffea234bb62e1fbc086ca0c");
    expect(tx.nonce).toStrictEqual(1);
    expect(tx.gasLimit).toStrictEqual(BigNumber.from(10_000));
    expect(tx.gasPrice).toStrictEqual(BigNumber.from(10_000));
    expect(tx.chainId).toStrictEqual(item.Legacy.chain_id);

    expect(msgHash).toBe(hash);
});

test("parse_transaction", async () => {
    let ser = {
        to: "0xe94f1fa4f27d9d288ffea234bb62e1fbc086ca0c",
        value: 0,
        data: [],
        chainId: 1,
        nonce: 1,
        gasPrice: BigNumber.from(10_000),
        gasLimit: 1
    };

    let raw_tx = ethers.utils.serializeTransaction(ser);

    let decoded = await can.parse_transaction(ethers.utils.arrayify(raw_tx));

    expect(decoded.Ok.Legacy).not.toBeUndefined();
    expect(BigNumber.from(decoded.Ok.Legacy.gas_limit)).toStrictEqual(BigNumber.from(1));
    expect(BigNumber.from(decoded.Ok.Legacy.gas_price)).toStrictEqual(BigNumber.from(10_000));
    expect(BigNumber.from(decoded.Ok.Legacy.nonce)).toStrictEqual(BigNumber.from(1));
    expect(BigNumber.from(decoded.Ok.Legacy.chain_id)).toStrictEqual(BigNumber.from(1));
});
