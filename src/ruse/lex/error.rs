use lex::Token;

use std::error::Error;
use std::fmt;

pub type LexResult = Result<Vec<Token>, LexError>;

#[derive(Debug)]
pub enum LexError {
    /// Character and location
    InvalidCharacter(char, i64),
    MalformedNumber(String),
}

impl Error for LexError {
    fn description(&self) -> &str {
        match *self {
            LexError::InvalidCharacter(..) => "invalid character",
            LexError::MalformedNumber(..) => "malformed number",
        }
    }
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LexError::InvalidCharacter(character, location) => {
                write!(f,
                       "invalid character '{}' at column {}",
                       character,
                       location)
            }
            LexError::MalformedNumber(ref number) => write!(f, "malformed number: {}", number),
        }
    }
}
