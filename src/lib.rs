mod admin;
mod connector;
mod consumer;
mod error;
mod fluvio;
#[allow(clippy::drop_non_drop)]
mod offset;
mod partition;
mod producer;
mod record;
mod smartmodule;
mod tableformat;
mod topic;

pub use crate::fluvio::Fluvio;
pub use admin::FluvioAdmin;
use connector::FluvioWebsocketConnector;
pub use consumer::{MultiplePartitionConsumer, PartitionConsumer, PartitionConsumerStream};
pub use error::FluvioError;
pub use offset::Offset;
pub use producer::TopicProducer;
pub use record::Record;

use js_sys::Reflect;
use wasm_bindgen::convert::FromWasmAbi;
use wasm_bindgen::JsValue;

pub(crate) fn generic_of_jsval<T: FromWasmAbi<Abi = u32>>(
    js: JsValue,
    classname: &str,
) -> Result<T, JsValue> {
    use js_sys::Object;
    let ctor_name = Object::get_prototype_of(&js).constructor().name();
    if ctor_name == classname {
        let ptr = Reflect::get(&js, &JsValue::from_str("ptr"))?;
        let ptr_u32: u32 = ptr.as_f64().ok_or(js)? as u32;
        let val = unsafe { T::from_abi(ptr_u32) };
        Ok(val)
    } else {
        Err(js)
    }
}
