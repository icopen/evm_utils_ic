import { getActor } from "./_common";
import { ethers } from "ethers";
import { expect } from "@jest/globals";


const can = await getActor(false, "evm_utils");

test("rlp_encode", async () => {
    let item = {
        values: [
            { Text: "test" },
            { Num: 64 },
            { Empty: null }
        ]
    }

    let encoded = await can.rlp_encode(item);
    // console.log(ethers.utils.hexlify(encoded.Ok));
    let decoded = ethers.utils.RLP.decode(encoded.Ok);

    expect(decoded.length).toBe(3);
});

test("rlp_decode", async () => {
    let bytes = ethers.utils.arrayify(ethers.utils.hexlify("0xc9845445737481e63201"));
    
    let decoded = await can.rlp_decode(bytes);
    expect(decoded.Ok.values.length).toBe(4);
});


