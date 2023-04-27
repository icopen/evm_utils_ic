import fs from 'fs';
import fetch, { Headers } from 'node-fetch'

import { TestContext } from 'lightic';
import { Principal } from '@dfinity/principal';

let context = undefined;

if (!globalThis.fetch) {
    globalThis.fetch = fetch
    globalThis.Headers = Headers
}

export async function initCanisters() {
    if (context !== undefined) return;
    context = new TestContext();
    let id = getCanisterId(false, "evm_utils");
    await context.deploy(".dfx/local/canisters/evm_utils/evm_utils.wasm", {
        id: Principal.from(id),
        candid: ".dfx/local/canisters/evm_utils/evm_utils.did"
    })
}


export function getCanisterId(useProd, canister) {
    let canisterId = null;

    if (useProd) {
        var data = JSON.parse(fs.readFileSync("../canister_ids.json"))
        canisterId = data[canister]["ic"];

    } else {
        var data = JSON.parse(fs.readFileSync(".dfx/local/canister_ids.json"))
        canisterId = data[canister]["local"];
    }

    console.log(canister + " Canister Id: " + canisterId);
    return canisterId;
}

export async function getIdlFactory(canisterName) {
    var path = "../.dfx/local/canisters/"+canisterName+"/"+canisterName+".did.js";
    let { idlFactory } = await import(path);

    return idlFactory;
}

//Returns actor for token canister
export async function getActor(useProd, canisterName) {
    await initCanisters();


    // let httpAgent = null;
    let canisterId = getCanisterId(useProd, canisterName);
    let actor = context.getAgent(Principal.anonymous()).getActor(canisterId);

    return actor;
}