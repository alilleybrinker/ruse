//! The entry point for running Ruse programs.
//!
//! The engine is the main interface between Ruse and Rust. It is where
//! Rust functions are registered, and where Ruse functions are run.

use read::read;
use read::parse;
use std::path::Path;
use std::fs::File;
use std::io::Read;

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

    /// Run the engine on a specific program.
    pub fn run<S: AsRef<str>>(&mut self, s: S) -> Result<String, parse::Error> {
        let syntax_tree = read(s).unwrap();
        Ok(syntax_tree.into())
    }

    /// Run the engine on a program from a file.
    ///
    /// TODO: Remove the unwrapping from this function.
    pub fn run_file<S: AsRef<Path>>(&mut self, s: S) -> Result<String, parse::Error> {
        let mut f = File::open(s).unwrap();
        let mut buffer = String::new();
        f.read_to_string(&mut buffer).unwrap();

        self.run(buffer)
    }
}
