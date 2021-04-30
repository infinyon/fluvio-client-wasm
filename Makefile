.PHONY: test

install-wasm-pack:
	 which wasm-pack || curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

build: install-wasm-pack
	wasm-pack build

test: install-wasm-pack
	wasm-pack test --headless --chrome
