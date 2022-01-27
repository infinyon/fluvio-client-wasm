//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use fluvio_client_wasm::FluvioAdmin;
use fluvio_client_wasm::PartitionConsumer;
use fluvio_client_wasm::TopicProducer;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);
use fluvio_client_wasm::{Fluvio, Offset};
use js_sys::Function;
use js_sys::Reflect;
use tracing::*;
use wasm_bindgen::convert::FromWasmAbi;
use wasm_bindgen::{JsCast, JsValue};

#[wasm_bindgen_test]
async fn base_test() {
    debug!("RUNNING BASE_TEST!");

    let url = "ws://localhost:3000".to_string();
    let fluvio = Fluvio::connect(url.clone()).await;
    assert!(fluvio.is_ok());
    let fluvio = fluvio.unwrap();
    let admin = wasm_bindgen_futures::JsFuture::from(fluvio.admin()).await;
    assert!(admin.is_ok());
    let admin = admin.unwrap();
    let admin: FluvioAdmin = generic_of_jsval(admin, "FluvioAdmin").unwrap();
    let topic = "my-integration-test".to_string();
    let _ = wasm_bindgen_futures::JsFuture::from(admin.create_topic(topic.clone(), 1)).await;

    let producer = wasm_bindgen_futures::JsFuture::from(fluvio.topic_producer(topic.clone())).await;
    assert!(producer.is_ok());
    let producer = producer.unwrap();
    let producer: TopicProducer = generic_of_jsval(producer, "TopicProducer").unwrap();
    let _ =
        wasm_bindgen_futures::JsFuture::from(producer.send(None, "value - 0".into())).await;

    let fluvio = Fluvio::connect(url).await;
    assert!(fluvio.is_ok());
    let fluvio = fluvio.unwrap();

    let consumer =
        wasm_bindgen_futures::JsFuture::from(fluvio.partition_consumer(topic.clone(), 0)).await;
    assert!(consumer.is_ok());
    let consumer = consumer.unwrap();
    let consumer: PartitionConsumer = generic_of_jsval(consumer, "PartitionConsumer").unwrap();

    let stream = consumer.stream(Offset::beginning()).await;
    assert!(stream.is_ok());
    let stream = stream.unwrap();
    let _ = wasm_bindgen_futures::JsFuture::from(stream.next()).await;
    for i in 1..10_usize {
        let value = format!("value - {:?}", i);
        let _ = wasm_bindgen_futures::JsFuture::from(producer.send(None, value.clone())).await;

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
    let _ = wasm_bindgen_futures::JsFuture::from(admin.delete_topic(topic)).await;
}

pub fn generic_of_jsval<T: FromWasmAbi<Abi = u32>>(
    js: JsValue,
    classname: &str,
) -> Result<T, JsValue> {
    use js_sys::Object;
    let ctor_name = Object::get_prototype_of(&js).constructor().name();
    if ctor_name == classname {
        let ptr = Reflect::get(&js, &JsValue::from_str("ptr"))?;
        let ptr_u32: u32 = ptr.as_f64().ok_or(JsValue::NULL)? as u32;
        let js_val = unsafe { T::from_abi(ptr_u32) };
        Ok(js_val)
    } else {
        Err(JsValue::NULL)
    }
}
