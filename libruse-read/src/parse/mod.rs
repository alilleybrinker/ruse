//! Generate a syntax tree from an input stream.

pub mod error;
pub mod expr;

pub use parse::error::{Error, Result};
use lex::token::Token;
use parse::expr::Expr;

/// Build an AST from a stream of tokens.
pub fn parse<V: AsRef<[Token]>>(v: V) -> Result {
    parse_expr(v.as_ref())
}

/// Parses a Ruse expression.
fn parse_expr(v: &[Token]) -> Result {
    if let Ok(a) = parse_atom(v) {
        Ok(a)
    } else if let Ok(a) = parse_integer(v) {
        Ok(a)
    } else if let Ok(a) = parse_float(v) {
        Ok(a)
    } else if let Ok(a) = parse_string(v) {
        Ok(a)
    } else if let Ok(a) = parse_bool(v) {
        Ok(a)
    } else if let Ok(a) = parse_list(v) {
        Ok(a)
    } else {
        Err(Error::InvalidProgram)
    }
}

/// Parses a Ruse atom
fn parse_atom(_v: &[Token]) -> Result {
    unimplemented!()
}

/// Parses a Ruse integer
fn parse_integer(_v: &[Token]) -> Result {
    unimplemented!()
}

/// Parses a Ruse float
fn parse_float(_v: &[Token]) -> Result {
    unimplemented!()
}

/// Parses a Ruse string
fn parse_string(_v: &[Token]) -> Result {
    unimplemented!()
}

/// Parses a Ruse bool
fn parse_bool(_v: &[Token]) -> Result {
    unimplemented!()
}

/// Parses a Ruse list
fn parse_list(_v: &[Token]) -> Result {
    unimplemented!()
}
