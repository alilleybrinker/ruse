//! Types for handling errors in the ruse engine.

use read;
use std::error;
use std::fmt;
use std::result;

/// The result of reading a string.
pub type Result = result::Result<String, Error>;

/// Indicates an error in lexing or parsing.
#[derive(Debug, PartialEq)]
pub enum Error {
    /// Indicates an error in reading.
    ReadError(read::error::Error),
}

impl error::Error for Error {
    /// Get a simple text description of what each error means.
    fn description(&self) -> &str {
        match *self {
            Error::ReadError(..) => "an error occured during reading",
        }
    }

    /// The underlying cause of the error.
    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::ReadError(ref error) => Some(error),
        }
    }
}

impl fmt::Display for Error {
    /// Print detailed error information.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::ReadError(ref error) => write!(f, "{}", error),
        }
    }
}

impl From<read::error::Error> for Error {
    /// Convert from a read::error into a top-level ruse Error.
    fn from(err: read::error::Error) -> Error {
        Error::ReadError(err)
    }
}
