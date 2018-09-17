//! Generate a syntax tree from an input stream.

pub mod error;
pub mod expr;

pub use parse::error::{Response, Result};
use parse::expr::*;
use lex::token::{Token, TokenKind};
use parse::expr::Expr;
use std::slice::Iter;
use std::iter::Peekable;

pub fn parse<V: AsRef<[Token]>>(v: V) -> Result {
    // TODO: Incorporate environment into parsing.
    let _env = Env::default();
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

fn parse_expr(v: &mut Tokens) -> Result {
    if let Ok(a) = parse_symbol(v) {
        return Ok(a);
    }

    if let Ok(a) = parse_number(v) {
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

    Err(Response::InvalidProgram)
}

fn parse_symbol(v: &mut Tokens) -> Result {
    let t = peek_or_stop!(v);

    if let TokenKind::Symbol(ref s) = t.kind {
        let kind = ExprKind::Symbol(s.clone());
        let expr = Expr::new(kind);
        return Ok(expr);
    }

    Err(Response::InvalidProgram)
}

fn parse_number(v: &mut Tokens) -> Result {
    let t = peek_or_stop!(v);

    if let TokenKind::Integer(i) = t.kind {
        let value = Number {
            kind: NumberKind::Int(i),
            exact: true,
        };
        let kind = ExprKind::Num(value);
        let expr = Expr::new(kind);
        return Ok(expr);
    }

    Err(Response::InvalidProgram)
}

fn parse_float(v: &mut Tokens) -> Result {
    let t = peek_or_stop!(v);

    if let TokenKind::Float(f) = t.kind {
        let value = Number {
            kind: NumberKind::Real(f),
            exact: true,
        };
        let kind = ExprKind::Num(value);
        let expr = Expr::new(kind);
        return Ok(expr);
    }

    Err(Response::InvalidProgram)
}

fn parse_string(v: &mut Tokens) -> Result {
    let t = peek_or_stop!(v);

    if let TokenKind::Str(ref s) = t.kind {
        let kind = ExprKind::Str(s.clone());
        let expr = Expr::new(kind);
        return Ok(expr);
    }

    Err(Response::InvalidProgram)
}

fn parse_bool(v: &mut Tokens) -> Result {
    let t = peek_or_stop!(v);

    if let TokenKind::Bool(b) = t.kind {
        let kind = ExprKind::Bool(b);
        let expr = Expr::new(kind);
        return Ok(expr);
    }

    Err(Response::InvalidProgram)
}

fn parse_list(v: &mut Tokens) -> Result {
    // Parse an opening delimiter, then a series of Ruse expressions
    // until you hit the matching closing delimiter. If you hit
    // a non-matching closing delimiter first, error out.
    let t = peek_or_stop!(v);

    if !t.kind.is_open_delim() {
        return Err(Response::InvalidProgram);
    }

    unimplemented!()
}

