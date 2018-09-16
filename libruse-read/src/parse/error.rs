//! Parser error and result types.

use parse::expr::Expr;

use std::error;
use std::fmt;
use std::result;

pub type Result = result::Result<Expr, Response>;

#[derive(Debug, PartialEq)]
pub enum Response {
    EmptyProgram,
    InvalidProgram,
    EndOfProgram,
}

impl error::Error for Response {
    fn description(&self) -> &str {
        match *self {
            Response::EmptyProgram => "empty program",
            Response::InvalidProgram => "invalid program",
            Response::EndOfProgram => "end of program",
        }
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", (self as &error::Error).description())
    }
}
