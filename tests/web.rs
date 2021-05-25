//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);
use fluvio_client_wasm::{
    Fluvio,
    FluvioError,
    Offset,
};
use log::*;

#[wasm_bindgen_test]
async fn base_test() {
    debug!("RUNNING BASE_TEST!");
    let fluvio_consumer = Fluvio::connect("ws://localhost:3000".into()).await;
    assert!(fluvio_consumer.is_ok());

    let fluvio = fluvio_consumer.unwrap();
    let consumer = fluvio.partition_consumer("foobar".into(), 0).await;
    assert!(consumer.is_ok());
    let consumer = consumer.unwrap();

    let stream = consumer.stream(Offset::beginning()).await;
    assert!(stream.is_ok());
    let stream = stream.unwrap();
    while let Some(val) = stream.next().await {
    }

    let fluvio_consumer = Fluvio::connect("ws://localhost:3000".into()).await;
    assert!(fluvio_consumer.is_ok());
    let fluvio = fluvio_consumer.unwrap();
}
