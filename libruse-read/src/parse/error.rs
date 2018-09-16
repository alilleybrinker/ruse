//! Parser error and result types.

use parse::expr::Expr;

use std::error;
use std::fmt;
use std::result;

/// A convenience type for returning either an `Expr` or a parsing error.
pub type Result = result::Result<Expr, Response>;

/// Indicates an error in parsing.
#[derive(Debug, PartialEq)]
pub enum Response {
    /// Indicates that the program is empty.
    EmptyProgram,
    /// Invalid program.
    InvalidProgram,
    /// End of program.
    EndOfProgram,
}

impl error::Error for Response {
    /// A simple textual description of the error.
    fn description(&self) -> &str {
        match *self {
            Response::EmptyProgram => "empty program",
            Response::InvalidProgram => "invalid program",
            Response::EndOfProgram => "end of program",
        }
    }
}

impl fmt::Display for Response {
    /// More detailed information about the error.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", (self as &error::Error).description())
    }
}
