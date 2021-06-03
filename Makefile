.PHONY: test

install-wasm-pack:
	 which wasm-pack || curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

build: install-wasm-pack
	wasm-pack build

build-dev:
	wasm-pack build --dev

test: install-wasm-pack
	wasm-pack test --firefox --headless

setup-websocat-sc:
	websocat -v --binary ws-l:127.0.0.1:3000 tcp:127.0.0.1:9003

setup-websocat-spu:
	websocat -v --binary ws-l:127.0.0.1:3001 tcp:127.0.0.1:9010

webpack-dev:
	npm install
	npm run webpack-dev
