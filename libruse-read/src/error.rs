//! Types for handling errors in reading.

use lex;
use parse;
use parse::expr::Expr;
use std::error;
use std::fmt;
use std::result;

/// The result of reading a string.
pub type Result = result::Result<Expr, Error>;

/// Indicates an error in lexing or parsing.
#[derive(Debug, PartialEq)]
pub enum Error {
    /// Indicates an error in lexing.
    LexError(lex::Error),
    /// Indicates an error in parsing.
    ParseError(parse::Response),
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

impl From<lex::Error> for Error {
    /// Convert from a lex::Error into a read::Error.
    fn from(err: lex::Error) -> Error {
        Error::LexError(err)
    }
}

impl From<parse::Response> for Error {
    /// Convert from a parse::Response into a read::Error.
    fn from(err: parse::Response) -> Error {
        Error::ParseError(err)
    }
}
