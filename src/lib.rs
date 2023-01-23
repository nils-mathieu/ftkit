//! A small set of utilities for newcomers learning Rust.

#![forbid(unsafe_op_in_unsafe_fn)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

mod input;
pub use self::input::*;

mod rand;
pub use self::rand::*;

mod args;
pub use self::args::*;
