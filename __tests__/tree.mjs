import { getActor } from "./_common";
import { BigNumber, ethers } from "ethers";
import { expect } from "@jest/globals";

import fs from "fs";
import { Trie } from "@ethereumjs/trie";

const can = await getActor(false, "evm_utils");


test("verify_proof", async () => {
    const trie = new Trie({ useKeyHashing: true })
    let data = JSON.parse(fs.readFileSync('./__tests__/proof.json'));

    let storageHash = Buffer.from(ethers.utils.arrayify(data.result.storageHash));
    let storageProof = data.result.storageProof[0];
    let value = storageProof.value;

    let padded_key = ethers.utils.hexZeroPad(storageProof.key, 32);

    //Prepare proof for trie
    let proof = [];
    for (let i of storageProof.proof) {
        proof.push(ethers.utils.arrayify(i));
    }

    let key_bytes = Buffer.from(ethers.utils.arrayify(padded_key));
    const result = await trie.verifyProof(storageHash, key_bytes, proof)
    //Output is RLP encoded, decode
    let result_data = ethers.utils.RLP.decode(result);
    expect(result_data).toBe(value);

    let can_result = await can.verify_proof(storageHash, key_bytes, proof);
    //Output is RLP encoded, decode
    let hex_result = ethers.utils.RLP.decode(Buffer.from(can_result.Ok[0]));

    expect(hex_result).toBe(value);
});
