//! The entry point for running Ruse programs.
//!
//! The engine is the main interface between Ruse and Rust. It is where
//! Rust functions are registered, and where Ruse functions are run.

use read::read;
use read::parse;

/// Eventually the Engine will store all Rust-side function bindings, and
/// provide a way for the user to register new bindings. There will also
/// be a utility to register bindings on the Ruse side, to be callable
/// from the Rust side.
pub struct Engine {}

impl Engine {
    /// Create a new Engine.
    pub fn new() -> Engine {
        Engine {}
    }

    /// TODO: Create an EvalError type.
    ///
    /// EvalError would probably be a wrapper around ParseError, along with some
    /// of its own failure variants, most notably failed function lookup,
    /// incorrect number of arguments, and invalid operation (type issues).
    pub fn eval<S: AsRef<str>>(&mut self, s: S) -> Result<String, parse::Error> {
        let syntax_tree = read(s.as_ref()).unwrap();
        Ok(syntax_tree.into())
    }
}
