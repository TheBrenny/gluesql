use error::OptionExt;
use error::ResultExt;
use gluesql_core::error::Error as GlueError;
use gluesql_core::error::Result as GlueResult;
use wasm_bindgen_futures::JsFuture;
use web_sys::wasm_bindgen::JsCast;
use web_sys::DomException;
use web_sys::{window, FileSystemDirectoryHandle};

mod error;

pub struct JsonWebStorage {
    fs_handle: FileSystemDirectoryHandle,
}

impl JsonWebStorage {
    pub async fn new() -> GlueResult<Self> {
        let window = window().ok_or_glue_error(error::JsonWebStorageError::WindowError)?;
        window.navigator().storage().persist().map_to_glue_err()?;
        let fs_handle = JsFuture::from(window.navigator().storage().get_directory()).await;

        let fs_handle = match fs_handle {
            Ok(js_value) => js_value.unchecked_into::<FileSystemDirectoryHandle>(),
            Err(e) => {
                return Err(GlueError::StorageMsg(
                    error::JsonWebStorageError::SecurityError(
                        e.unchecked_into::<DomException>()
                            .as_string()
                            .unwrap_or_default(),
                    )
                    .to_string(),
                ))
            }
        };

        Ok(Self { fs_handle })
    }
}
