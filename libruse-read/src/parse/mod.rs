//! Generate a syntax tree from an input stream.

pub mod error;
pub mod expr;

pub use parse::error::{Response, Result};
use lex::token::{Token, TokenKind};
use parse::expr::Expr;
use std::slice::Iter;
use std::iter::Peekable;

/// Build an AST from a stream of tokens.
pub fn parse<V: AsRef<[Token]>>(v: V) -> Result {
    let mut i = v.as_ref().iter().peekable();
    parse_expr(&mut i)
}

macro_rules! unwrap_or_return {
    ( $e:expr, $r:expr ) => {
        match $e {
            Some(x) => x,
            None => return $r,
        }
    }
}

macro_rules! peek_or_stop {
    ( $e:expr ) => {
        unwrap_or_return!($e.peek(), Err(Response::EndOfProgram));
    }
}

// A shorthand type, to make the below functions more readable.
//
// The iterator needs to be peekable so we can avoid moving the iterator
// forward in the case of a failed parse.
type Tokens<'a> = Peekable<Iter<'a, Token>>;

/// Parses a Ruse expression.
fn parse_expr(v: &mut Tokens) -> Result {
    while let Some(_) = v.peek() {
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
    }

    Err(Response::InvalidProgram)
}

/// Parses a Ruse ident
fn parse_ident(v: &mut Tokens) -> Result {
    let t = peek_or_stop!(v);

    if let TokenKind::Ident(ref s) = t.kind {
        return Ok(Expr::Ident(s.clone()));
    }

    Err(Response::InvalidProgram)
}

/// Parses a Ruse integer
fn parse_integer(v: &mut Tokens) -> Result {
    let t = peek_or_stop!(v);

    if let TokenKind::Integer(i) = t.kind {
        return Ok(Expr::Integer(i));
    }

    Err(Response::InvalidProgram)
}

/// Parses a Ruse float
fn parse_float(v: &mut Tokens) -> Result {
    let t = peek_or_stop!(v);

    if let TokenKind::Float(f) = t.kind {
        return Ok(Expr::Float(f));
    }

    Err(Response::InvalidProgram)
}

/// Parses a Ruse string
fn parse_string(v: &mut Tokens) -> Result {
    let t = peek_or_stop!(v);

    if let TokenKind::Str(ref s) = t.kind {
        return Ok(Expr::Str(s.clone()));
    }

    Err(Response::InvalidProgram)
}

/// Parses a Ruse bool
fn parse_bool(v: &mut Tokens) -> Result {
    let t = peek_or_stop!(v);

    if let TokenKind::Bool(b) = t.kind {
        return Ok(Expr::Bool(b));
    }

    Err(Response::InvalidProgram)
}

/// Parses a Ruse list
fn parse_list(_v: &mut Tokens) -> Result {
    // Parse an opening delimiter, then a series of Ruse expressions
    // until you hit the matching closing delimiter. If you hit
    // a non-matching closing delimiter first, error out.
    unimplemented!()
}


#[cfg(test)]
mod tests {
    use super::*;
    use lex::*;

    #[test]
    fn test_parse_ident() {
        let tokens = lex("g").unwrap();
        let mut t_iter = tokens.iter().peekable();
        let result = parse_ident(&mut t_iter);
        assert_eq!(Ok(Expr::Ident("g".to_string())), result);
    }

    #[test]
    fn test_parse_integer() {
        let tokens = lex("1").unwrap();
        let mut t_iter = tokens.iter().peekable();
        let result = parse_integer(&mut t_iter);
        assert_eq!(Ok(Expr::Integer(1)), result);
    }

    #[test]
    fn test_parse_string() {
        let tokens = lex("\"hello\"").unwrap();
        let mut t_iter = tokens.iter().peekable();
        let result = parse_string(&mut t_iter);
        assert_eq!(Ok(Expr::Str("hello".to_string())), result);
    }

    #[test]
    fn test_parse_bool() {
        let tokens = lex("#t").unwrap();
        let mut t_iter = tokens.iter().peekable();
        let result = parse_bool(&mut t_iter);
        assert_eq!(Ok(Expr::Bool(true)), result);
    }
}

