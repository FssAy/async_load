use async_load::{
    ds_map::*,
    types::*,
};

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub unsafe extern "C" fn MyFunction() -> GMLDouble {
    use std::thread::*;
    use std::time::Duration;

    // Spawn new thread.
    spawn(|| {
        // Wait for 1 second.
        sleep(Duration::from_secs(1));

        // Create and dispatch the map.
        let mut map = DSMap::new();
        map.add_double("key1", 21.37);
        map.add_string("key2", "Hello from Rust!");
        map.dispatch(EventType::Social);
    });

    GMLDouble::none()
}
