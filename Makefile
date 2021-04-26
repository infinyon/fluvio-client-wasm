.PHONY: test

build:
	wasm-pack build

test:
	wasm-pack test --headless --firefox
