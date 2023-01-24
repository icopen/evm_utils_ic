cargo build --target wasm32-unknown-unknown
ic-cdk-optimizer ./target/wasm32-unknown-unknown/release/evm_utils.wasm -o ./target/wasm32-unknown-unknown/release/evm_utils_out.wasm