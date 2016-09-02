//! Read a Ruse program into a data structure which may be evaluated.

pub mod lex;
pub mod parse;
pub mod util;

use read::lex::Lexer;
use read::parse::Parser;

/// Read a string representing a Ruse program into a syntax tree representation
/// of that program.
pub fn read<S: AsRef<str>>(s: S) -> parse::Result {
    let tokens = Lexer::lex(s.as_ref()).unwrap();
    Parser::parse(&tokens)
}
