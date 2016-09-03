//! Read a Ruse program into a data structure which may be evaluated.

pub mod error;
pub mod lex;
pub mod parse;

use read;
use read::lex::Lexer;
use read::parse::Parser;

/// Read a string representing a Ruse program into a syntax tree representation
/// of that program.
pub fn read<S: AsRef<str>>(s: S) -> read::error::Result {
    let tokens = try!(Lexer::lex(s));
    Ok(try!(Parser::parse(&tokens)))
}
