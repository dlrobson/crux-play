//! Contains shared code for the Crux app and the FFI bindings.

use crux_core::{Core, bridge::Bridge};

use crate::Counter;

/// The main interface used by the shell
#[cfg_attr(feature = "uniffi", derive(uniffi::Object))]
#[cfg_attr(feature = "wasm_bindgen", wasm_bindgen::prelude::wasm_bindgen)]
pub struct CoreFFI {
    core: Bridge<Counter>,
}

impl Default for CoreFFI {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg_attr(feature = "uniffi", uniffi::export)]
#[cfg_attr(feature = "wasm_bindgen", wasm_bindgen::prelude::wasm_bindgen)]
impl CoreFFI {
    /// Create a new `CoreFFI` instance.
    #[cfg_attr(feature = "uniffi", uniffi::constructor)]
    #[cfg_attr(
        feature = "wasm_bindgen",
        wasm_bindgen::prelude::wasm_bindgen(constructor)
    )]
    #[must_use]
    pub fn new() -> Self {
        Self {
            core: Bridge::new(Core::new()),
        }
    }

    /// Send an event to the app and return the effects.
    /// # Panics
    /// If the event cannot be deserialized.
    /// In production you should handle the error properly.
    #[must_use]
    pub fn update(&self, data: &[u8]) -> Vec<u8> {
        let mut effects = Vec::new();
        match self.core.update(data, &mut effects) {
            Ok(()) => effects,
            #[expect(clippy::panic, reason = "This is example code")]
            Err(e) => panic!("{e}"),
        }
    }

    /// Get the current `ViewModel`.
    /// # Panics
    /// If the view cannot be serialized.
    /// In production you should handle the error properly.
    #[must_use]
    pub fn view(&self) -> Vec<u8> {
        let mut view_model = Vec::new();
        match self.core.view(&mut view_model) {
            Ok(()) => view_model,
            #[expect(clippy::panic, reason = "This is example code")]
            Err(e) => panic!("{e}"),
        }
    }
}
