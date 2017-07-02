//! Ruse is an embedded Scheme for Rust. It is currently in the very early
//! stages, and should not be used for anything.
//!
//! At the moment, Ruse is little more than a lexer and a work-in-progress
//! parser. Long term, the goal is for Ruse to be an R7RS small-specification
//! compliant version of Scheme for embedding in Rust programs. Ruse code will
//! be able to call Rust code, and Rust code will be able to call Ruse. Their
//! relationship should be akin to the relationship between Lua and C.
//!
//! This is obviously an amibitious project, and I don't know when/if Ruse will
//! reach this point. But it's sure fun to play with!

#![deny(missing_docs)]

extern crate libruse_read as read;
extern crate libruse_eval as eval;
extern crate libruse_print as print;

pub mod error;

use std::path::Path;
use std::fs::File;
use std::io::Read;

use error::Result;
use read::read;
use eval::eval;
use print::print;

/// The entry point for running Ruse programs.
///
/// The engine is the main interface between Ruse and Rust. It is where
/// Rust functions are registered, and where Ruse functions are run.
///
/// Eventually the Engine will store all Rust-side function bindings, and
/// provide a way for the user to register new bindings. There will also
/// be a utility to register bindings on the Ruse side, to be callable
/// from the Rust side.
#[derive(Default)]
pub struct Engine {}

impl Engine {
    /// Create a new Engine.
    pub fn new() -> Engine {
        Default::default()
    }

    /// Run the engine on a specific program.
    pub fn run<S: AsRef<str>>(&mut self, s: S) -> Result {
        let r = read(s).unwrap();
        let e = eval(r).unwrap();
        let p = print(e).unwrap();
        Ok(p)
    }

    /// Run the engine on a program from a file.
    ///
    /// TODO: Remove the unwrapping from this function.
    pub fn run_file<S: AsRef<Path>>(&mut self, s: S) -> Result {
        let mut f = File::open(s).unwrap();
        let mut buffer = String::new();
        f.read_to_string(&mut buffer).expect("could not read file");
        self.run(buffer)
    }
}
