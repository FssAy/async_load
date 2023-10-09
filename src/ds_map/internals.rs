use super::*;

/// Macro used for defining logic to register the internal GMS2 calls.
///
/// # Args
/// - `$name`: Name that will be used to create statics and functions (uppercase).
/// - `$fun_type`: Type of the internal GMS2 function. (src/ds_map/types.rs)
macro_rules! define_internal_call {
    ($name:ty, $fun_type:ty) => { paste! {
        // Holds an address of the internal GMS2 function.
        static mut [<ADDR_ $name>]: FnAddr = 0;
        // Holds a pointer to the internal GMS2 function.
        pub(super) static mut [<FN_ $name>]: *mut $fun_type = std::ptr::null_mut();

        // Used to save the internal GMS2 function.
        pub(super) unsafe fn [<register_ $name:lower>](addr: FnAddr) {
            [<ADDR_ $name>] = addr;
            [<FN_ $name>] = (&[<ADDR_ $name>] as *const FnAddr) as *mut $fun_type;
        }
    } }
}

define_internal_call!(EVENT_PERFORM_ASYNC, EventPerformAsync);
define_internal_call!(DS_MAP_CREATE, DsMapCreate);
define_internal_call!(DS_MAP_ADD_DOUBLE, DsMapAddDouble);
define_internal_call!(DS_MAP_ADD_STRING, DsMapAddString);
