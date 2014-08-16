RustyEmitter
============
RustyEmitter is a basic implementation of a simple emitter.
The module expose a `Events` trait with the `on`, `off` and `emit` methods,
and a default implementation of that trait, called Emitter.

In order to use it, you must `use` the trait in your code.

The Emitter can register multiple callbacks for the same event,
and it will pass *a copy* of an HashMap<String, String> to those.

Build and Test
==============
The project uses cargo
build using `cargo build` and test it using the `cargo test` commands.
