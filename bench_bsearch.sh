export ZKWASM_K=22
export ZKWASM_BSEARCH_FILE="wasm/bsearch_4000000.wasm"
echo "testing" $ZKWASM_BSEARCH_FILE "with K=" $ZKWASM_K
cargo test bench::binary_search::tests::test_binary_search --release --features cuda -- --nocapture

export ZKWASM_K=21
export ZKWASM_BSEARCH_FILE="wasm/bsearch_2000000.wasm"
echo "testing" $ZKWASM_BSEARCH_FILE "with K=" $ZKWASM_K
cargo test bench::binary_search::tests::test_binary_search --release --features cuda -- --nocapture

export ZKWASM_K=20
export ZKWASM_BSEARCH_FILE="wasm/bsearch_1000000.wasm"
echo "testing" $ZKWASM_BSEARCH_FILE "with K=" $ZKWASM_K
cargo test bench::binary_search::tests::test_binary_search --release --features cuda -- --nocapture

export ZKWASM_K=19
export ZKWASM_BSEARCH_FILE="wasm/bsearch_500000.wasm"
echo "testing" $ZKWASM_BSEARCH_FILE "with K=" $ZKWASM_K
cargo test bench::binary_search::tests::test_binary_search --release --features cuda -- --nocapture


export ZKWASM_K=18
export ZKWASM_BSEARCH_FILE="wasm/bsearch_200000.wasm"
echo "testing" $ZKWASM_BSEARCH_FILE "with K=" $ZKWASM_K
cargo test bench::binary_search::tests::test_binary_search --release --features cuda -- --nocapture
