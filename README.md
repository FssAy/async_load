# Async Load
This crate contains functionality to trigger GMS2 async events and pass data into them by using a `ds_map` structure inside a global `async_load` variable.

# About
**GMS2** (_Game Maker Studio 2_) is an engine for building games on all kind of platforms that uses **GML** (_Game Maker Language_).

It's possible to enhance its functionality with extensions (_custom libraries_). <br>
This crate helps with building one that can "call `GML` from `Rust`".

# How To Use
In order to be able to perform any kind of operation, it's necessary to define an external `RegisterCallbacks` function. <br>
It can be found in `src/lib.rs`. 

To activate it simply enable the `extension` feature:
```toml
[dependencies.ds_map]
version = "1.0.2"
features = [
    "extension",
]
```

_(You can also keep the feature disabled and define `RegisterCallbacks` manually)_

Then while inside of your GMS2 project, open your extension and add a new function as seen on the image bellow:
<img src="https://raw.githubusercontent.com/FssAy/async_load/master/assets/fn_def.png">

Do not call this function, it will be executed automatically by the GMS2.

This will register all the necessary callbacks and allow you to use the `DSMap` structure.

# Example
Rust:
```rust
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
```

GMS2 object `Create` event:
```gml
show_debug_message("[*] Start");
MyFunction();
show_debug_message("[*] Finished");
```

GMS2 object `Async - Social` event:
```gml
show_debug_message("[*] Async - Social");
show_debug_message(async_load[? "key1"]);
show_debug_message(async_load[? "key2"]);
```

Output:
```
[*] Start
[*] Finished

(...) <some other info>

[*] Async - Social
21.37
Hello from Rust!
```
