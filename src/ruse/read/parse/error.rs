//! Parser error and result types.

use read::lex;
use read::parse::SyntaxTree;
use std::error;
use std::fmt;
use std::result;

/// A convenience type for returning either a SyntaxTree or a parsing error.
pub type Result = result::Result<SyntaxTree, Error>;

/// Indicates an error in parsing.
#[derive(Debug)]
pub enum Error {
    /// A wrapper around an underlying lexer error.
    BadInput(lex::Error),
    /// Indicates that there were no outer parentheses.
    NoEnclosingParens,
    /// Indicates that the parentheses were not matched.
    UnmatchedParens,
}

impl error::Error for Error {
    /// A simple textual description of the error.
    fn description(&self) -> &str {
        match *self {
            Error::BadInput(ref lex_error) => lex_error.description(),
            Error::NoEnclosingParens => "no enclosing parens",
            Error::UnmatchedParens => "unmatched parens",
        }
    }

    /// Indicate that a lex error may be wrapped.
    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::BadInput(ref lex_error) => Some(lex_error),
            Error::NoEnclosingParens | Error::UnmatchedParens => None,
        }
    }
}

impl fmt::Display for Error {
    /// More detailed information about the error.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", (self as &error::Error).description())
    }
}

impl From<lex::Error> for Error {
    /// Convert from a lex::Error into a parse::Error.
    fn from(err: lex::Error) -> Error {
        Error::BadInput(err)
    }
}
