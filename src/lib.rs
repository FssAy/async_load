//! # Async Load
//! This crate contains functionality to trigger GMS2 async events and pass data into them by using a `ds_map` structure inside a global `async_load` variable.
//!
//! # How To Use
//! In order to see how to use this library with your GMS2 extension, visit the [github repo](https://github.com/FssAy/async_load).
//!
//! # Example
//! Rust:
//! ```rust
//! #[no_mangle]
//! #[allow(non_snake_case, unused_variables)]
//! pub unsafe extern "C" fn MyFunction() -> GMLDouble {
//!     use std::thread::*;
//!     use std::time::Duration;
//!
//!     // Spawn new thread.
//!     spawn(|| {
//!         // Wait for 1 second.
//!         sleep(Duration::from_secs(1));
//!
//!         // Create and dispatch the map.
//!         let mut map = DSMap::new();
//!         map.add_double("key1", 21.37);
//!         map.add_string("key2", "Hello from Rust!");
//!         map.dispatch(EventType::Social);
//!     });
//!
//!     GMLDouble::none()
//! }
//! ```

#[macro_use]
extern crate paste;

#[cfg(test)]
mod tests;
pub mod types;
pub mod ds_map;

/// Export function that should be defined inside of GMS2 extension editor.
///
/// This function will be called automatically by the GMS2.
/// Do not call it manually.
///
/// Enable `extension` feature when building a "cdylib",
/// or leave it disabled if you want to define this function manually.
#[cfg(feature = "extension")]
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
unsafe extern "C" fn RegisterCallbacks(
    event_perform_async: *mut std::os::raw::c_void,
    ds_map_create: *mut std::os::raw::c_void,
    ds_map_add_double: *mut std::os::raw::c_void,
    ds_map_add_string: *mut std::os::raw::c_void,
) -> types::GMLDouble {
    use types::*;
    use ds_map::*;

    DSMap::register_calls(
        event_perform_async,
        ds_map_create,
        ds_map_add_double,
        ds_map_add_string
    );

    GMLDouble::none()
}
