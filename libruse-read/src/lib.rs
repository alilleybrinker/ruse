//! Read a Ruse program into a data structure which may be evaluated.

#![deny(missing_docs)]

pub mod lex;
pub mod parse;
pub mod error;

use error::Result;
use lex::lex;
use parse::parse;

/// Read a string representing a Ruse program into a syntax tree representation
/// of that program.
pub fn read<S: AsRef<str>>(program: S) -> Result {
    let tokens = lex(program)?;
    let ast = parse(&tokens)?;
    Ok(ast)
}
