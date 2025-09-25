build:
	cargo build --release

run_with_log:
	RUST_BACKTRACE=1 ./target/release/solochain-template-node -ldebug --dev

run_with_chain_state:
	./target/release/solochain-template-node --dev --base-path ./my-chain-state/

purge_chain:
	./target/release/solochain-template-node purge-chain --dev

build_benchmark:
	cargo build --release --features runtime-benchmarks

benchmark:
	frame-omni-bencher v1 benchmark pallet \
	--runtime ./target/release/wbuild/pba-runtime/pba_runtime.compact.compressed.wasm \
	--pallet "pallet-multisig" \
	--extrinsic "" \
	--output pallets/multisig/src/weights/multisig.rs