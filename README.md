## About

This is the browser wasm client for
[fluvio](https://github.com/infinyon/fluvio). This is meant to be used via
javascript in a web browser.

## 🚴 Usage

This project is currently setup to be used via rollup.

## Development

Setup fluvio with a `--local` cluster installation locally then run the
following:

* `make run-fluvio-websocket-proxy`

### Hot Reloading

To use hotreloading for the contents of
[`js/index.js`](https://github.com/infinyon/fluvio-client-wasm/blob/main/js/index.js)
* `make webpack-dev`

Now go edit `js/index.js` to try your work.

### Rust Tests

Integration tests are defined in the `tests` directory.

* `make run-fluvio-websocket-proxy` in one terminal.
* `make test`  will run the rust tests in a headless browser.
