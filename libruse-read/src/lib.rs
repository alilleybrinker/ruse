#![feature(cell_update)]

pub mod lex;
pub mod parse;
pub mod error;
use error::Result;
use lex::lex;
use parse::parse;

pub fn read<S: AsRef<str>>(program: S) -> Result {
    let tokens = lex(program)?;
    let ast = parse(tokens)?;
    Ok(ast)
}

