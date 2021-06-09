.PHONY: test

install-wasm-pack:
	 which wasm-pack || curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

build: install-wasm-pack
	wasm-pack build

build-dev:
	wasm-pack build --dev

test: install-wasm-pack
	wasm-pack test --firefox --headless

fluvio-websocket-proxy:
	cargo run -- ./fluvio-websocket-proxy/Cargo.toml

webpack-dev:
	npm install
	npm run webpack-dev
