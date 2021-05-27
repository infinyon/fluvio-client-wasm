
use fluvio::Offset as NativeOffset;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Offset {
    pub(crate) inner: NativeOffset,
}

#[wasm_bindgen]
impl Offset {
    pub fn from_beginning(offset: u32) -> Self {
        NativeOffset::from_beginning(offset).into()
    }
    pub fn beginning() -> Self {
        NativeOffset::beginning().into()
    }
    pub fn from_end(offset: u32) -> Self {
        NativeOffset::from_end(offset).into()
    }
    pub fn end() -> Self {
        NativeOffset::end().into()
    }
    /*
    pub fn absolute(index: i64) -> Result<Self, FluvioError>{
        NativeOffset::absolute(index).map(Offset::from).map_err(|e| FluvioError::from(e))
    }
    */
}


impl From<NativeOffset> for Offset {
    fn from(inner: NativeOffset) -> Self {
        Self { inner }
    }
}
