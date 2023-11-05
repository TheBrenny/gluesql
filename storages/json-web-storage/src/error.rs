use web_sys::js_sys::Object;
use web_sys::wasm_bindgen::JsCast;

use {gluesql_core::error::Error, thiserror::Error};

pub trait ResultExt<T, JsValue> {
    fn map_to_glue_err(self) -> Result<T, Error>;
}
impl<T, JsValue> ResultExt<T, JsValue> for Result<T, JsValue> {
    fn map_to_glue_err(self) -> Result<T, Error> {
        self.map_err(|e| e.unchecked_into::<Object>())
            .map_err(Error::StorageMsg)
    }
}
pub trait OptionExt<T, E: ToString> {
    fn ok_or_glue_error(self, error: E) -> Result<T, Error>;
}
impl<T, E: ToString> OptionExt<T, E> for Option<T> {
    fn ok_or_glue_error(self, error: E) -> Result<T, Error> {
        self.ok_or_else(|| error.to_string())
            .map_err(Error::StorageMsg)
    }
}

#[derive(Error, Debug)]
pub enum JsonWebStorageError {
    #[error("window error")]
    WindowError,
    #[error("security error: {0}")]
    SecurityError(String),
}
