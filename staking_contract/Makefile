prepare:
	rustup target add wasm32-unknown-unknown

build-contract:
	cd staking_contract && cargo build --release --target wasm32-unknown-unknown
	wasm-strip staking_contract/target/wasm32-unknown-unknown/release/staking_contract.wasm 2>/dev/null | true

	cd ../erc20/erc20-token && cargo build --release --target wasm32-unknown-unknown
	wasm-strip ../erc20/erc20-token/target/wasm32-unknown-unknown/release/staking_contract.wasm 2>/dev/null | true

test-only:
	cd staking_contract_tests && cargo test

copy-wasm-file-to-test:
	cp staking_contract/target/wasm32-unknown-unknown/release/*.wasm staking_contract_tests/wasm
	cp ../erc20/target/wasm32-unknown-unknown/release/erc20_token.wasm staking_contract_tests/wasm/erc20.wasm

test: build-contract copy-wasm-file-to-test test-only

clippy:
	cd staking_contract && cargo clippy --all-targets --all -- -D warnings

check-lint: clippy
	cd staking_contract && cargo fmt --all -- --check

lint: clippy
	cd staking_contract && cargo fmt --all

clean:
	cargo clean
	rm -rf staking_contract_tests/wasm/*.wasm
