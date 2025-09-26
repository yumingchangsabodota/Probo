build:
	cargo build --release

run:
	RUST_BACKTRACE=1 ./target/release/solochain-template-node -ldebug --dev

run_with_chain_state:
	./target/release/solochain-template-node --dev --base-path ./my-chain-state/

purge_chain:
	./target/release/solochain-template-node purge-chain --dev

build_benchmark:
	cargo build --release --features runtime-benchmarks

benchmark:
	frame-omni-bencher v1 benchmark pallet \
	--runtime ./target/release/wbuild/solochain-template-runtime/solochain_template_runtime.compact.compressed.wasm \
	--pallet "pallet-issuance-proof" \
	--extrinsic "" \
	--output pallets/issuance-proof/src/weights/issuance-proof.rs

test:
	cargo test -p pallet-issuance-proof -- --nocapture

rustup_switch:
	rustup default ${version} && \
	rustup target add wasm32-unknown-unknown --toolchain ${version}-aarch64-apple-darwin && \
	rustup component add rust-src
