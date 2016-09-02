//! Lexing error and result types.

use lex::Token;
use std::error;
use std::fmt;
use std::result;

/// The result of lexing. Either a vector of tokens if the lexing is successful
/// or an Error if it's not.
pub type Result = result::Result<Vec<Token>, Error>;

/// A lexing error, indicating some problem with the contents of the input
/// stream.
#[derive(Debug, PartialEq)]
pub enum Error {
    /// Character and location
    InvalidCharacter(char, usize),
    /// The almost-number.
    MalformedNumber(String),
}

impl error::Error for Error {
    /// Get a simple text description of what each error means.
    fn description(&self) -> &str {
        match *self {
            Error::InvalidCharacter(..) => "invalid character",
            Error::MalformedNumber(..) => "malformed number",
        }
    }
}

impl fmt::Display for Error {
    /// Print detailed error information.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::InvalidCharacter(character, location) => {
                write!(f,
                       "invalid character '{}' at column {}",
                       character,
                       location)
            }
            Error::MalformedNumber(ref number) => write!(f, "malformed number: {}", number),
        }
    }
}
