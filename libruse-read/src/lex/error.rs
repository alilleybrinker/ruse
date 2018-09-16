use lex::token::{Token, Location};
use std::error;
use std::fmt;
use std::result;

pub type Result = result::Result<Vec<Token>, Error>;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidCharacter(char, Location),
    MalformedNumber(String, Location),
    InvalidLiteral(String, Location),
    InvalidEscapeSequence(String, Location),
}

impl error::Error for Error {
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::InvalidCharacter(character, location) => {
                write!(
                    f,
                    "invalid character '{}' at {}",
                    character,
                    location
                )
            }
            Error::MalformedNumber(ref number, location) => {
                write!(f, "malformed number '{}' at '{}'", number, location)
            }
            Error::InvalidLiteral(ref string, location) => {
                write!(f, "invalid literal '{}' at '{}'", string, location)
            }
            Error::InvalidEscapeSequence(ref string, location) => {
                write!(
                    f,
                    "invalid escape sequence '{}' at '{}'",
                    string,
                    location
                )
            }
        }
    }
}
