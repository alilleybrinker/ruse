//! Read a Ruse program into a data structure which may be evaluated.

pub mod error;
pub mod lex;
pub mod parse;

use read::error::Result;
use read::lex::Lex;
use read::parse::Parse;

/// Read a string representing a Ruse program into a syntax tree representation
/// of that program.
pub fn read<S: AsRef<str>>(s: S) -> Result {
    Ok(s.lex()?.parse()?)
}
