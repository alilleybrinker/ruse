//! Stores function bindings and provides a way for the user to register new
//! bindings, on either the Ruse or Rust side. This is also the entry point
//! to running Ruse programs.

use lex::Lexer;
use parse::{self, Parser};

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
        let tokens = Lexer::lex(s.as_ref()).unwrap();

        println!("{:?}", tokens);

        let syntax_tree = Parser::parse(&tokens).unwrap();

        Ok(syntax_tree.into())
    }
}
