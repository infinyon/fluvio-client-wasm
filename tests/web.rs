//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);
use fluvio_client_wasm::{Fluvio, FluvioError, Offset, Record};
use js_sys::Function;
use js_sys::Reflect;
use tracing::*;
use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
use wasm_bindgen::{JsCast, JsValue};

#[wasm_bindgen_test]
async fn base_test() {
    debug!("RUNNING BASE_TEST!");

    let url = "ws://localhost:3000".to_string();
    let topic = "my-integration-test".to_string();
    let fluvio = Fluvio::connect(url.clone()).await;
    assert!(fluvio.is_ok());
    let fluvio = fluvio.unwrap();

    let producer = fluvio.topic_producer(topic.clone()).await;
    assert!(producer.is_ok());
    let producer = producer.unwrap();
    let _ =
        wasm_bindgen_futures::JsFuture::from(producer.send("".into(), "value - 0".into())).await;

    let fluvio = Fluvio::connect(url).await;
    assert!(fluvio.is_ok());
    let fluvio = fluvio.unwrap();

    let consumer = fluvio.partition_consumer(topic, 0).await;
    assert!(consumer.is_ok());
    let consumer = consumer.unwrap();
    let stream = consumer.stream(Offset::beginning()).await;
    assert!(stream.is_ok());
    let stream = stream.unwrap();
    let _ = wasm_bindgen_futures::JsFuture::from(stream.next()).await;
    for i in 1..10 {
        let value = format!("value - {:?}", i);
        let _ = wasm_bindgen_futures::JsFuture::from(producer.send("".into(), value.clone())).await;

        let next = wasm_bindgen_futures::JsFuture::from(stream.next()).await;
        assert!(next.is_ok());
        let next = next.unwrap();

        let ret_value = Reflect::get(&next, &"valueString".into());
        assert!(ret_value.is_ok());
        let ret_value = ret_value.unwrap().dyn_into::<Function>();
        assert!(ret_value.is_ok());
        let ret_value = ret_value.unwrap();

        let ret_value = ret_value.call0(&next);
        assert!(ret_value.is_ok());
        let ret_value = ret_value.unwrap();
        let ret_value = ret_value.as_string();

        assert_eq!(ret_value, Some(value));
    }
}
