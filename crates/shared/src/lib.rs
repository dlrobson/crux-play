//! Contains shared code for the Crux app and the FFI bindings.

mod app;
pub mod ffi;

pub use app::Counter;

#[cfg(feature = "uniffi")]
const _: () = assert!(
    uniffi::check_compatible_version("0.29.4"),
    "please use uniffi v0.29.4"
);
#[cfg(feature = "uniffi")]
uniffi::setup_scaffolding!();
