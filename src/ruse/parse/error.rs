use parse::{SyntaxTree, Token};
use std::error::Error;
use std::fmt;

pub type ParseResult = Result<SyntaxTree, ParseError>;

#[derive(Debug)]
pub enum ParseError {
    BadInput(LexError),
}

impl Error for ParseError {
    fn description(&self) -> &str {
        match *self {
            ParseError::BadInput(ref lex_error) => lex_error.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl From<LexError> for ParseError {
    fn from(err: LexError) -> ParseError {
        ParseError::BadInput(err)
    }
}

pub type LexResult<'a> = Result<Vec<Token<'a>>, LexError>;

#[derive(Debug)]
pub enum LexError {
    NoEnclosingParens,
    UnmatchedParens,
    /// Character and location
    InvalidCharacter(char, i64),
}

impl Error for LexError {
    fn description(&self) -> &str {
        match *self {
            LexError::NoEnclosingParens => "no enclosing parens",
            LexError::UnmatchedParens => "unmatched parens",
            LexError::InvalidCharacter(..) => "invalid character",
        }
    }
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LexError::NoEnclosingParens => write!(f, "no enclosing parens"),
            LexError::UnmatchedParens => write!(f, "unmatched parens"),
            LexError::InvalidCharacter(character, location) => {
                write!(f,
                       "invalid character '{}' at column {}",
                       character,
                       location)
            }
        }
    }
}
