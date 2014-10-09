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
[dependencies.emitter]

git = "https://github.com/kentaromiura/RustyEmitter.git"
```

to your `Cargo.toml` file.

to use it like
```
extern crate emitter;
use emitter::{Events, Emitter};


...

// create a new emitter instance
  let mut emitter = Emitter::new();
// listen to the "IT WORKS" event
  emitter.on(String::from_str("IT WORKS"),|data| print!("IT WORKS, {}", data));
// fire the "IT WORKS" event with an empty HashMap;
  emitter.emit(String::from_str("IT WORKS"), HashMap::new());

// fire it again passing some more data
  let mut datas : HashMap<String, String> = HashMap::new();
  datas.insert("some data", "here");
  emitter.emit(String::from_str("IT WORKS", & mut datas));
```
