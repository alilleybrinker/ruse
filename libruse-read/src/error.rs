use lex;
use parse;
use parse::expr::Expr;
use std::error;
use std::fmt;
use std::result;

pub type Result = result::Result<Expr, Error>;

#[derive(Debug, PartialEq)]
pub enum Error {
    LexError(lex::Error),
    ParseError(parse::Response),
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::LexError(..) => "an error occured during lexing",
            Error::ParseError(..) => "an error occured during parsing",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::LexError(ref error) => Some(error),
            Error::ParseError(ref error) => Some(error),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::LexError(ref error) => write!(f, "{}", error),
            Error::ParseError(ref error) => write!(f, "{}", error),
        }
    }
}

impl From<lex::Error> for Error {
    fn from(err: lex::Error) -> Error {
        Error::LexError(err)
    }
}

impl From<parse::Response> for Error {
    fn from(err: parse::Response) -> Error {
        Error::ParseError(err)
    }
}
