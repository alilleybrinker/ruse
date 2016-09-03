//! Parser error and result types.

use read::parse::SyntaxTree;
use std::error;
use std::fmt;
use std::result;

/// A convenience type for returning either a SyntaxTree or a parsing error.
pub type Result = result::Result<SyntaxTree, Error>;

/// Indicates an error in parsing.
#[derive(Debug, PartialEq)]
pub enum Error {
    /// Indicates that there were no outer parentheses.
    NoEnclosingParens,
    /// Indicates that the parentheses were not matched.
    UnmatchedParens,
}

impl error::Error for Error {
    /// A simple textual description of the error.
    fn description(&self) -> &str {
        match *self {
            Error::NoEnclosingParens => "no enclosing parens",
            Error::UnmatchedParens => "unmatched parens",
        }
    }
}

impl fmt::Display for Error {
    /// More detailed information about the error.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", (self as &error::Error).description())
    }
}
