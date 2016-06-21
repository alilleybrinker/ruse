use lex;
use parse::SyntaxTree;
use std::error;
use std::fmt;
use std::result;

pub type Result = result::Result<SyntaxTree, Error>;

#[derive(Debug)]
pub enum Error {
    BadInput(lex::Error),
    NoEnclosingParens,
    UnmatchedParens,
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::BadInput(ref lex_error) => lex_error.description(),
            Error::NoEnclosingParens => "no enclosing parens",
            Error::UnmatchedParens => "unmatched parens",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::BadInput(ref lex_error) => Some(lex_error),
            Error::NoEnclosingParens | Error::UnmatchedParens => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", (self as &error::Error).description())
    }
}

impl From<lex::Error> for Error {
    fn from(err: lex::Error) -> Error {
        Error::BadInput(err)
    }
}
