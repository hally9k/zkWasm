export ZKWASM_K=22
export ZKWASM_FIBONACCI_DEP=19
echo "testing fibonacci with dep=" $ZKWASM_FIBONACCI_DEP " K=" $ZKWASM_K
cargo test bench::fibonacci::tests::test_fibonacci --release --features cuda -- --nocapture


export ZKWASM_K=21
export ZKWASM_FIBONACCI_DEP=18
echo "testing fibonacci with dep=" $ZKWASM_FIBONACCI_DEP " K=" $ZKWASM_K
cargo test bench::fibonacci::tests::test_fibonacci --release --features cuda -- --nocapture


export ZKWASM_K=20
export ZKWASM_FIBONACCI_DEP=16
echo "testing fibonacci with dep=" $ZKWASM_FIBONACCI_DEP " K=" $ZKWASM_K
cargo test bench::fibonacci::tests::test_fibonacci --release --features cuda -- --nocapture

export ZKWASM_K=19
export ZKWASM_FIBONACCI_DEP=15
echo "testing fibonacci with dep=" $ZKWASM_FIBONACCI_DEP " K=" $ZKWASM_K
cargo test bench::fibonacci::tests::test_fibonacci --release --features cuda -- --nocapture

export ZKWASM_K=18
export ZKWASM_FIBONACCI_DEP=13
echo "testing fibonacci with dep=" $ZKWASM_FIBONACCI_DEP " K=" $ZKWASM_K
cargo test bench::fibonacci::tests::test_fibonacci --release --features cuda -- --nocapture
