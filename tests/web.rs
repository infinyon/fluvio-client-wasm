//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);
use fluvio_client_wasm::{
    Fluvio,
    FluvioError,
};
use log::*;

#[wasm_bindgen_test]
async fn base_test() {
    debug!("RUNNING BASE_TEST!");
    let fluvio = Fluvio::connect().await.unwrap();
}
