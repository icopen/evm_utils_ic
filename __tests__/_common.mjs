import { Actor, HttpAgent } from '@dfinity/agent';

import fs from 'fs';
import fetch, { Headers } from 'node-fetch'

if (!globalThis.fetch) {
    globalThis.fetch = fetch
    globalThis.Headers = Headers
}


// export const idlFactory = ({ IDL }) => {
//     return IDL.Service({
//         __get_candid_interface_tmp_hack: IDL.Func([], [IDL.Text], ["query"]),
//     });
// };
// export const init = ({ IDL }) => {
//     return [];
// };


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

    let httpAgent = null;
    let canisterId = getCanisterId(useProd, canisterName);

    if (useProd) {
        var host = "https://boundary.ic0.app/"; //ic

        httpAgent = new HttpAgent({ host });
    } else {
        const host = "http://127.0.0.1:4943"; //local

        httpAgent = new HttpAgent({ host });
        httpAgent.fetchRootKey();
    }

    let idlFactory = await getIdlFactory(canisterName);

    const actor = Actor.createActor(idlFactory, {
        agent: httpAgent,
        canisterId: canisterId,
    });

    return actor;
}