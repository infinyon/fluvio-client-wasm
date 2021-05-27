
use std::cell::RefCell;
use std::rc::Rc;

use crate::FluvioError;
use fluvio::metadata::topic::TopicSpec;
use fluvio::metadata::objects::Metadata;
use fluvio::FluvioAdmin as NativeFluvioAdmin;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;
use js_sys::Promise;

#[wasm_bindgen]
pub struct FluvioAdmin {
    inner: Rc<RefCell<NativeFluvioAdmin>>,
}
#[wasm_bindgen]
impl FluvioAdmin {
    pub fn list_topic(&mut self) -> Promise {
        let rc = self.inner.clone();
        future_to_promise(async move {
            let _topics = rc.borrow_mut().list::<TopicSpec, _>(vec![]).await;
            unimplemented!();

        })
    }
}

impl From<NativeFluvioAdmin> for FluvioAdmin {
    fn from(inner: NativeFluvioAdmin) -> Self {
        Self { inner: Rc::new(RefCell::new(inner)) }
    }
}
