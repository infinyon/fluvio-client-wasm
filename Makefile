.PHONY: test

build:
	wasm-pack build

build-dev:
	wasm-pack build --dev

test:
	wasm-pack test --firefox --headless

watch:
	#cargo watch -w ./ -w=../fluvio/src/{client,socket}/ -s 'make test' -w ../ws_stream_wasm/
	cargo watch -w ./ -w=../fluvio/src/{client,socket}/ -s 'make build-dev' -w ../ws_stream_wasm/

setup-websocat-sc:
	websocat -v --binary ws-l:127.0.0.1:3000 tcp:127.0.0.1:9003

setup-websocat-spu:
	websocat -v --binary ws-l:127.0.0.1:3001 tcp:127.0.0.1:9010
