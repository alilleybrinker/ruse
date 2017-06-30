//! Read a Ruse program into a data structure which may be evaluated.

use read::error::Result;
use read::lex::lex::Lex;
use read::parse::parse::Parse;

/// Read a string representing a Ruse program into a syntax tree representation
/// of that program.
pub fn read<S: AsRef<str>>(s: S) -> Result {
    Ok(s.lex()?.parse()?)
}
