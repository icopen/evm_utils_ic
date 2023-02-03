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

# Contributing

# Security

# Funding