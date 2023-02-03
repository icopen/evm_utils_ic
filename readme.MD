# Overview

This canister "EVM Utility Canister" provides multiple functions required to create and parse EVM compliant transactions. It was created in order to give possibility for Motoko devs to interact with EVM chains.

Link [https://icdevs.org/bounties/2023/01/09/28-EVM-Utility-Canister-Rust.html]

# Functions

## RLP

- [x] `rlp_encode` - Based on provided data returns rlp encoded bytes
- [x] `rlp_decode` - decodes RLP bytes in to an object.


## Transaction

- [x] `create_transaction` - Works with an unsigned transaction, returns encoded transaction bytes and hash that is used for signing the transaction
- [x] `parse_transaction` - Works with both signed and non signed transactions, returns object of decoded transaction. Calculates hash and from for transaction (sender is not included, however it can be recovered from transaction data and signature)

## EVM Verification

- [x] `verify_proof` - Verifies if send hash is a part of a tree, returns retrieved data from proof. You need to send a root hash, proof and key.

## Hashing

- [x] `keccak256` - Hashes incoming data using keccak and returns hash

## Utils
For now I have decided to skip functions related to private keys processing. I do believe it is not secure to pass public keys.

- [x] `recover_public_key` - recovers public key from ethereum style recoverable signature (65 bytes), equivalent to ecrecover
- [x] `is_valid_public` - checks if public key is valid
- [x] `is_valid_signature` - validates ECDSA signature
- [x] `pub_to_address` - converts public key to ethereum address

# Getting started
This package was published as a cargo crate, to use it type
```
cargo add ic_evm_utils
````

If you want to compile the package, clone the repo and run
```
cargo build --target wasm32-unknown-unknown
```

There is also prepared build script `build.sh` that builds and optimizes the wasm package using `ic-cdk-optimizer` (you need to install this first)

# Testing
There are two types of tests prepared. First they are written in Rust to check if in rust everything matches our assumptions. Then there are JS tests that validate output of prepared functions against battle tested ethers library.

## Rust tests
To run tests written in Rust
```
cargo test
```

## Js tests
First you need to run local replica
```
dfx start --clean --background
```

Then you need to deploy canisters
```
dfx deploy
```

Once deployment is complete you can run tests in js
```
yarn test
```

# Funding
This library was initially incentivized by [ICDevs](https://icdevs.org/). You can view more about the bounty on the [website](https://icdevs.org/bounties/2023/01/09/28-EVM-Utility-Canister-Rust.html).