//! Types for handling errors in reading.

use read::{lex, parse};
use std::error;
use std::fmt;
use std::result;

/// The result of reading a string.
pub type Result = result::Result<parse::syntax_tree::SyntaxTree, Error>;

/// Indicates an error in lexing or parsing.
#[derive(Debug, PartialEq)]
pub enum Error {
    /// Indicates an error in lexing.
    LexError(lex::error::Error),
    /// Indicates an error in parsing.
    ParseError(parse::error::Error),
}

impl error::Error for Error {
    /// Get a simple text description of what each error means.
    fn description(&self) -> &str {
        match *self {
            Error::LexError(..) => "an error occured during lexing",
            Error::ParseError(..) => "an error occured during parsing",
        }
    }

    /// The underlying cause of the error.
    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::LexError(ref error) => Some(error),
            Error::ParseError(ref error) => Some(error),
        }
    }
}

impl fmt::Display for Error {
    /// Print detailed error information.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::LexError(ref error) => write!(f, "{}", error),
            Error::ParseError(ref error) => write!(f, "{}", error),
        }
    }
}

impl From<lex::error::Error> for Error {
    /// Convert from a lex::Error into a read::Error.
    fn from(err: lex::error::Error) -> Error {
        Error::LexError(err)
    }
}

impl From<parse::error::Error> for Error {
    /// Convert from a parse::Error into a read::Error.
    fn from(err: parse::error::Error) -> Error {
        Error::ParseError(err)
    }
}
