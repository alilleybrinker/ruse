//! Generate a syntax tree from an input stream.

pub mod error;
pub mod expr;

pub use parse::error::{Error, Result};
use lex::token::{Token, TokenKind};
use parse::expr::Expr;
use std::slice::Iter;
use std::iter::Peekable;

/// Build an AST from a stream of tokens.
pub fn parse<V: AsRef<[Token]>>(v: V) -> Result {
    let mut i = v.as_ref().iter().peekable();
    parse_expr(&mut i)
}

// A shorthand type, to make the below functions more readable.
//
// The iterator needs to be peekable so we can avoid moving the iterator
// forward in the case of a failed parse.
type Tokens<'a> = Peekable<Iter<'a, Token>>;

/// Parses a Ruse expression.
fn parse_expr(v: &mut Tokens) -> Result {
    if let Ok(a) = parse_ident(v) {
        return Ok(a);
    }

    if let Ok(a) = parse_integer(v) {
        return Ok(a);
    }

    if let Ok(a) = parse_float(v) {
        return Ok(a);
    }

    if let Ok(a) = parse_string(v) {
        return Ok(a);
    }

    if let Ok(a) = parse_bool(v) {
        return Ok(a);
    }

    if let Ok(a) = parse_list(v) {
        return Ok(a);
    }

    Err(Error::InvalidProgram)
}

/// Parses a Ruse ident
fn parse_ident(v: &mut Tokens) -> Result {
    // Check if the next token is an ident. If it is, succeed.
    // Otherwise, error out.
    //
    // Make sure to only have the iterator progress if the next
    // token is actually an ident.

    if let Some(t) = v.peek() {
        if let TokenKind::Ident(ref s) = t.kind {
            return Ok(Expr::Ident(s.clone()));
        }
    }

    unreachable!()
}

/// Parses a Ruse integer
fn parse_integer(_v: &mut Tokens) -> Result {
    // Check if the next token is an integer. If it is, succeed.
    // Otherwise, error out.
    //
    // Make sure to only have the iterator progress if the next
    // token is actually an integer.
    unimplemented!()
}

/// Parses a Ruse float
fn parse_float(_v: &mut Tokens) -> Result {
    // Check if the next token is a float. If it is, succeed.
    // Otherwise, error out.
    //
    // Make sure to only have the iterator progress if the next
    // token is actually a float.
    unimplemented!()
}

/// Parses a Ruse string
fn parse_string(_v: &mut Tokens) -> Result {
    // Check if the next token is a string. If it is, succeed.
    // Otherwise, error out.
    //
    // Make sure to only have the iterator progress if the next
    // token is actually a string.
    unimplemented!()
}

/// Parses a Ruse bool
fn parse_bool(_v: &mut Tokens) -> Result {
    // Check if the next token is a bool. If it is, succeed.
    // Otherwise, error out.
    //
    // Make sure to only have the iterator progress if the next
    // token is actually a bool.
    unimplemented!()
}

/// Parses a Ruse list
fn parse_list(_v: &mut Tokens) -> Result {
    // Parse an opening delimiter, then a series of Ruse expressions
    // until you hit the matching closing delimiter. If you hit
    // a non-matching closing delimiter first, error out.
    unimplemented!()
}
