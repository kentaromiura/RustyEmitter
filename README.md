RustyEmitter
============

RustyEmitter is a basic implementation of a simple emitter.
The module expose a `Events` trait with the `on`, `off` and `emit` methods,
and a default implementation of that trait, called Emitter.

In order to use it, you must `use` the trait in your code.

The Emitter can register multiple callbacks for the same event,
and it will pass *a mutable reference* of an HashMap<String, T> to those.

Since it's a mutable reference the function consuming the emitter can modify the HashMap, hence allowing a 2 channel communication between the event dispatcher and the code listening to the event

Build and Test
==============

The project uses cargo
build using `cargo build` and test it using the `cargo test` commands.

Cargo configuration
===================

just add
```
[dependencies.RustyEmitter]
version = "2.1.0"
```

to your `Cargo.toml` file.

to use it like
```
extern crate RustyEmitter;
use RustyEmitter::{Events, Emitter};
use std::collections::HashMap;

fn main(){  
  let (mut emitter, callback) = (
    // create a new emitter instance
    Emitter::new(),
    // creating the handler in the same lifetime 
    &mut |data:& mut HashMap<String, String>| { 
      println!("IT WORKS!");
      for (key, value) in data {
          println!("{}: {}", key, value);
      }
    }
  );
  // listen to the "IT WORKS" event
  emitter.on("IT WORKS".to_string(), callback);
  // fire the "IT WORKS" event with an empty HashMap;
  emitter.emit("IT WORKS".to_string(), & mut HashMap::new());

  // fire it again passing some more data
  let mut datas : HashMap<String, String> = HashMap::new();
  datas.insert("some data".to_string(), "here".to_string());
  emitter.emit("IT WORKS".to_string(), & mut datas);
}

```
