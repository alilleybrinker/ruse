use parse::{Parser, ParseError};

// Eventually the Engine will store all Rust-side function bindings, and
// provide a way for the user to register new bindings. There will also
// be a utility to register bindings on the Ruse side, to be callable
// from the Rust side.
pub struct Engine {}

impl Engine {
    pub fn new() -> Engine {
        Engine {}
    }

    // TODO: Create an EvalError type.
    //
    // EvalError would probably be a wrapper around ParseError, along with some
    // of its own failure variants, most notably failed function lookup,
    // incorrect number of arguments, and invalid operation (type issues).
    pub fn eval<S: AsRef<str>>(&mut self, s: S) -> Result<String, ParseError> {
        let mut parser = Parser::new();
        let result: String = try!(parser.parse(s.as_ref())).into();
        Ok(result)
    }
}
