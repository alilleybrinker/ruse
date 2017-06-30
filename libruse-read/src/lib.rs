//! Read a Ruse program into a data structure which may be evaluated.

pub mod lex;
pub mod parse;
pub mod error;

use error::Result;
use lex::Lex;
use parse::Parse;

/// Read a string representing a Ruse program into a syntax tree representation
/// of that program.
pub fn read<S: AsRef<str>>(s: S) -> Result {
    Ok(s.lex()?.parse()?)
}
