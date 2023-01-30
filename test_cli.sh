#!/bin/bash

set -e
set -x

# rm -rf output

# Single test
#RUST_LOG=info cargo run --release -- --function zkmain --output ./output --wasm g1024.wasm setup

#RUST_LOG=info cargo run --release -- --function zkmain --output ./output --wasm g1024.wasm single-prove --public 43:i64 --private 0x00030001000302010001000103030001000100030001000300010002010003000100030003010003030100:bytes-packed
#RUST_LOG=info cargo run --release -- --function zkmain --output ./output --wasm g1024.wasm single-prove --public 30:i64 --private 0x000301020003020002010003010001000302010003020002010300020001:bytes-packed
RUST_LOG=info cargo run --release -- --function zkmain --output ./output --wasm g1024.wasm single-prove --public 1:i64 --private 0x00:bytes-packed
RUST_LOG=info cargo run --release -- --function zkmain --output ./output --wasm g1024.wasm single-verify --public 43:i64 --proof output/zkwasm.0.transcript.data
exit 0
RUST_LOG=info cargo run --release -- --function bsearch --output ./output --wasm wasm/bsearch_64.wasm aggregate-prove --public 3:i64
RUST_LOG=info cargo run --release -- --function bsearch --output ./output --wasm wasm/bsearch_64.wasm aggregate-verify --proof output/aggregate-circuit.0.transcript.data  --instances output/aggregate-circuit.0.instance.data
RUST_LOG=info cargo run --release -- --function bsearch --output ./output --wasm wasm/bsearch_64.wasm solidity-aggregate-verifier --proof output/aggregate-circuit.0.transcript.data  --instances output/aggregate-circuit.0.instance.data
