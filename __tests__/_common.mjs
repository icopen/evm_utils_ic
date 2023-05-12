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
    return await context.deploy("target/wasm32-unknown-unknown/release/evm_utils.wasm")
}

//Returns actor for token canister
export async function getActor(useProd, canisterName) {
    let canister = await initCanisters();

    let actor = context.getAgent(Principal.anonymous()).getActor(canister);

    return actor;
}