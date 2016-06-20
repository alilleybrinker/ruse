use parse::SyntaxTree;
use lex::LexError;

use std::error::Error;
use std::fmt;

pub type ParseResult = Result<SyntaxTree, ParseError>;

#[derive(Debug)]
pub enum ParseError {
    BadInput(LexError),
    NoEnclosingParens,
    UnmatchedParens,
}

impl Error for ParseError {
    fn description(&self) -> &str {
        match *self {
            ParseError::BadInput(ref lex_error) => lex_error.description(),
            ParseError::NoEnclosingParens => "no enclosing parens",
            ParseError::UnmatchedParens => "unmatched parens",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ParseError::BadInput(ref lex_error) => Some(lex_error),
            ParseError::NoEnclosingParens |
            ParseError::UnmatchedParens => None,
        }
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
