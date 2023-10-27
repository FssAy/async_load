mod types;
mod internals;

use std::ffi::CString;
use std::os::raw::{c_char, c_double, c_int, c_void};
use crate::types::GMLDouble;
pub use types::*;
use internals::*;

/// GML data structure used for passing data into an async event.
///
/// Read more here:
/// https://manual.yoyogames.com/The_Asset_Editors/Object_Properties/Async_Events.htm
#[derive(Debug, Eq, PartialEq)]
pub struct DSMap {
    pub(crate) map_id: c_int,
}

impl DSMap {
    /// Function used for registering internal calls into GMS2.
    /// It is designated to run inside of exported function `RegisterCallbacks`.
    ///
    /// # Args
    /// All arguments are internal GMS2 functions.
    /// - `event_perform_async`: Calls a specific async event.
    /// - `ds_map_create`: Creates new map data structure.
    /// - `ds_map_add_double`: Adds a `double` value into the map.
    /// - `ds_map_add_string`: Adds a `string` value into the map.
    ///
    /// # Safety
    /// DO NOT call this function on your own as it might cause undefined behaviour!
    /// All the methods of `DSMap` structure calls the internal GMS2 functions passed as the arguments here,
    /// so when the callbacks are not registered and any of the method is executed, undefined behaviour will occur.
    pub unsafe fn register_calls(
        event_perform_async: *const c_void,
        ds_map_create: *const c_void,
        ds_map_add_double: *const c_void,
        ds_map_add_string: *const c_void,
    ) {
        register_event_perform_async(event_perform_async as FnAddr);
        register_ds_map_create(ds_map_create as FnAddr);
        register_ds_map_add_double(ds_map_add_double as FnAddr);
        register_ds_map_add_string(ds_map_add_string as FnAddr);
    }

    /// Creates a new map data structure.
    ///
    /// # Safety
    /// Do not execute this method if the internal GMS2 functions were not
    /// registered using `DSMap::register_calls`.
    pub unsafe fn new() -> Self {
        let map_id = (*FN_DS_MAP_CREATE)(0);
        DSMap { map_id }
    }

    /// Creates a wrapper around existing map data structure.
    ///
    /// This method could be used to perform some work on a ds_map created
    /// inside GMS2 by passing its ID.
    ///
    /// # Args
    /// - `map_id`: ID of the existing map data structure.
    ///
    /// # Safety
    /// There is no checks in place that will make sure the provided ID is valid,
    /// or that the map will not be destroyed while in use.
    ///
    /// It's easy to break something when working on a data structure created inside of the GMS2,
    /// so use it only if you know what you are doing.
    pub fn new_existing(map_id: c_int) -> Self {
        DSMap { map_id }
    }

    /// Adds a new `double` value to the map on a specific key.
    ///
    /// # Args
    /// - `key`: The key of the value to add.
    /// - `value`: The `double` value to add to the map.
    ///
    /// # Return
    /// Returns `bool` when the operation was successful.
    /// This method might fail if the map doesn't exist, or the key was already inside of the map.
    ///
    /// # Safety
    /// Do not execute this method if the internal GMS2 functions were not
    /// registered using `DSMap::register_calls`.
    pub unsafe fn add_double(&mut self, key: impl ToString, value: GMLDouble) -> bool {
        // Will never fail if key is a valid String
        let key_cstr = CString::new(
            key.to_string()
        ).unwrap();

        (*FN_DS_MAP_ADD_DOUBLE)(
            self.map_id,
            key_cstr.as_ptr(),
            value,
        )
    }

    /// Adds a new `string` value to the map on a specific key.
    ///
    /// # Args
    /// - `key`: The key of the value to add.
    /// - `value`: The `string` value to add to the map.
    ///
    /// # Return
    /// Returns `bool` when the operation was successful.
    /// This method might fail if the map doesn't exist, or the key was already inside of the map.
    ///
    /// # Safety
    /// Do not execute this method if the internal GMS2 functions were not
    /// registered using `DSMap::register_calls`.
    pub unsafe fn add_string(&mut self, key: impl ToString, value: impl ToString) -> bool {
        // Will never fail if key is a valid String
        let key_cstr = CString::new(
            key.to_string()
        ).unwrap();

        // Will never fail if value is a valid String
        let value_cstr = CString::new(
            value.to_string()
        ).unwrap();

        (*FN_DS_MAP_ADD_STRING)(
            self.map_id,
            key_cstr.as_ptr(),
            value_cstr.as_ptr(),
        )
    }

    /// Adds a new raw data to the map on a specific key.
    ///
    /// # Args
    /// - `key`: The key of the value to add.
    /// - `value`: Pointer to the data.
    ///
    /// # Return
    /// Returns `bool` when the operation was successful.
    /// This method might fail if the map doesn't exist, or the key was already inside of the map.
    ///
    /// # Safety
    /// Do not execute this method if the internal GMS2 functions were not
    /// registered using `DSMap::register_calls`.
    ///
    /// Make sure the raw value that the pointer points to is valid and will not be dealocated by Rust.
    pub unsafe fn add_raw(&mut self, key: impl ToString, value: *const u8) -> bool {
        // Will never fail if key is a valid String
        let key_cstr = CString::new(
            key.to_string()
        ).unwrap();

        (*FN_DS_MAP_ADD_STRING)(
            self.map_id,
            key_cstr.as_ptr(),
            value as *const _,
        )
    }

    /// Triggers a specific async event inside of the GMS2 and passes the map into it.
    ///
    /// This map will be available in GMS2 as `async_load`.
    ///
    /// Once the map is dispatched it will be destroyed by the GMS2,
    /// so any operation on it after this point is not possible.
    ///
    /// # Args
    /// - `event`: Type of the event to trigger.
    ///
    /// # Safety
    /// Do not execute this method if the internal GMS2 functions were not
    /// registered using `DSMap::register_calls`.
    pub unsafe fn dispatch(self, event: EventType) {
        (*FN_EVENT_PERFORM_ASYNC)(self.map_id, event as c_int);
    }
}
