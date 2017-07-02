//! Parser error and result types.

use parse::expr::Expr;

use std::error;
use std::fmt;
use std::result;

/// A convenience type for returning either an `Expr` or a parsing error.
pub type Result = result::Result<Expr, Error>;

// TODO: Make these errors more useful.
/// Indicates an error in parsing.
#[derive(Debug, PartialEq)]
pub enum Error {
    /// Indicates that the program is empty.
    EmptyProgram,
    /// Invalid program.
    InvalidProgram,
}

impl error::Error for Error {
    /// A simple textual description of the error.
    fn description(&self) -> &str {
        match *self {
            Error::EmptyProgram => "empty program",
            Error::InvalidProgram => "invalid program",
        }
    }
}

impl fmt::Display for Error {
    /// More detailed information about the error.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", (self as &error::Error).description())
    }
}
