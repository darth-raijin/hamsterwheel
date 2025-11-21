# hamsterwheel
A collection of cache data structures for Rust, focused on ergonomics, observability, testability, and 100% safe Rust.

The aim is to provide cache primitives that are simple to use, predictable in behavior, and easy to instrument in real services. Every cache is built with clear APIs, first-class metrics, and no hidden complexity. If you're looking for production-friendly caches with strong correctness guarantees and good developer experience, this crate is for you.

## Features

- **100% safe Rust**  
  Enforced with `#![forbid(unsafe_code)]` at the crate root.

- **Ergonomic API**  
  Simple, predictable method names and clean semantics. No surprises.

- **Observable by default**  
  Built-in metrics for key performance indicators.

- **Highly testable**  
  Deterministic behavior, stable iteration, and introspection helpers make it easy to write precise assertions.

