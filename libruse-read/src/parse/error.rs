//! Parser error and result types.

use parse::expr::Expr;

use std::error;
use std::fmt;
use std::result;

/// A convenience type for returning either an `Expr` or a parsing error.
pub type Result = result::Result<Expr, Error>;

/// Indicates an error in parsing.
#[derive(Debug, PartialEq)]
pub enum Error {
    /// Indicates that there were no outer parentheses.
    NoEnclosingParens,
    /// Indicates that the parentheses were not matched.
    UnmatchedParens,
    /// Indicates that the program is empty.
    EmptyProgram,
    /// Indicates an identifier was found in an unexpected position.
    UnexpectedIdentifier(String),
}

impl error::Error for Error {
    /// A simple textual description of the error.
    fn description(&self) -> &str {
        match *self {
            Error::NoEnclosingParens => "no enclosing parens",
            Error::UnmatchedParens => "unmatched parens",
            Error::EmptyProgram => "empty program",
            Error::UnexpectedIdentifier(..) => "unexpected identifier",
        }
    }
}

impl fmt::Display for Error {
    /// More detailed information about the error.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::UnexpectedIdentifier(ref s) => write!(f, "unexpected identifier '{}'", s),
            _ => write!(f, "{}", (self as &error::Error).description()),
        }
    }
}
