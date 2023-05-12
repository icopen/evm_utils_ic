import fs from 'fs';
import fetch, { Headers } from 'node-fetch'

import { TestContext } from 'lightic';
import { Principal } from '@dfinity/principal';
import { HttpAgent, Actor } from '@dfinity/agent';
import { idlFactory } from './evm_utils.did';

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
export async function getActor() {
    if (process.env.USE_DFX !== undefined) {
        let agent = new HttpAgent({
            host: 'http://127.0.0.1:4943'
        })
        await agent.fetchRootKey()

        let actor = Actor.createActor(idlFactory, {
            agent: agent,
            canisterId: 'rrkah-fqaaa-aaaaa-aaaaq-cai',
        });

        return actor;
    } else {
        let canister = await initCanisters();
        let actor = context.getAgent(Principal.anonymous()).getActor(canister);

        return actor;
    }
}