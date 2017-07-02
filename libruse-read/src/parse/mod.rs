//! Generate a syntax tree from an input stream.

pub mod error;
pub mod expr;

pub use parse::error::{Error, Result};
use lex::token::{Token, TokenKind};
use parse::expr::Expr;

/// Struct to track the balance of parens
struct Parens {
    count: i64,
}

impl Parens {
    /// Construct a new paren tracker
    fn new() -> Self {
        Parens { count: 0 }
    }

    /// Return whether parens are balanced
    fn matching(&self) -> bool {
        self.count != 0
    }

    /// Return whether parens are not balanced
    fn not_matching(&self) -> bool {
        !self.matching()
    }
}

/// Build an AST from a stream of tokens.
pub fn parse(v: &[Token]) -> Result {
    let ast: Option<Expr> = None;
    let mut parens = Parens::new();

    for token in v {
        if parens.matching() && !token.kind.is_open_delim() {
            return Err(Error::NoEnclosingParens);
        }

        match token.kind {
            TokenKind::OpenDelim(..) => parens.count += 1,
            TokenKind::CloseDelim(..) => parens.count -= 1,
            TokenKind::Ident(ref s) => {
                if parens.matching() {
                    return Err(Error::UnexpectedIdentifier(s.to_owned()));
                }

                // uhhh
            }
            _ => {}
        }
    }

    if parens.not_matching() {
        return Err(Error::UnmatchedParens);
    }

    match ast {
        Some(a) => Ok(a),
        None => Err(Error::EmptyProgram),
    }
}
