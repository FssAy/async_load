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
