//! Lexing error and result types.

use lex::token::Token;

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
    MalformedNumber(String, usize),
    /// An almost-literal
    InvalidLiteral(String, usize),
    /// An almost-escape sequence
    InvalidEscapeSequence(String, usize),
}

impl error::Error for Error {
    /// Get a simple text description of what each error means.
    fn description(&self) -> &str {
        match *self {
            Error::InvalidCharacter(..) => "invalid character",
            Error::MalformedNumber(..) => "malformed number",
            Error::InvalidLiteral(..) => "invalid literal",
            Error::InvalidEscapeSequence(..) => "invalid escape sequence",
        }
    }
}

impl fmt::Display for Error {
    /// Print detailed error information.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::InvalidCharacter(character, location) => {
                write!(
                    f,
                    "invalid character '{}' at column {}",
                    character,
                    location
                )
            }
            Error::MalformedNumber(ref number, location) => {
                write!(f, "malformed number '{}' at column '{}'", number, location)
            }
            Error::InvalidLiteral(ref string, location) => {
                write!(f, "invalid literal '{}' at column '{}'", string, location)
            }
            Error::InvalidEscapeSequence(ref string, location) => {
                write!(
                    f,
                    "invalid escape sequence '{}' at column '{}'",
                    string,
                    location
                )
            }
        }
    }
}
