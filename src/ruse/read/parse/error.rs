//! Parser error and result types.

use read::parse::expr::Expr;
use std::error;
use std::fmt;
use std::result;

/// A convenience type for returning either an `Expr` or a parsing error.
pub type Result = result::Result<Expr, Error>;

/// Indicates an error in parsing.
#[derive(Debug, PartialEq)]
pub enum Error {
    /// Indicates that there were no outer parentheses.
    /// 表示没有外圆括号。
    NoEnclosingParens,
    /// Indicates that the parentheses were not matched.
    /// 表示括号不匹配。
    UnmatchedParens,
    /// Indicates that the parentheses are empty.
    /// 表示括号内容为空。
    EmptyParens,
}

impl error::Error for Error {
    /// A simple textual description of the error.
    fn description(&self) -> &str {
        match *self {
            Error::NoEnclosingParens => "no enclosing parens",
            Error::UnmatchedParens => "unmatched parens",
            Error::EmptyParens => "empty parens",
        }
    }
}

impl fmt::Display for Error {
    /// More detailed information about the error.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", (self as &error::Error).description())
    }
}
