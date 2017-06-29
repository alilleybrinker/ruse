//! Generate a syntax tree from an input stream.

pub mod error;
pub mod expr;

use read::lex::token::{Token, TokenKind};
use read::parse::error::{Result, Error};
use read::parse::expr::Expr;

/// Parse an input string, returning a Ruse expression that can be evaluated.
pub trait Parse {
    /// Parse a type into an AST.
    fn parse(&self) -> Result;
}

struct Parens {
    count: i64,
}

impl Parens {
    fn new() -> Self {
        Parens {
            count: 0
        }
    }

    fn matching(&self) -> bool {
        if self.count != 0 { false }
        else { true }
    }

    fn not_matching(&self) -> bool {
        !self.matching()
    }
}

impl Parse for Vec<Token> {
    /// Parse a vector of tokens into an AST.
    fn parse(&self) -> Result {
        let ast: Option<_> = None;
        let mut parens = Parens::new();

        for token in self {
            if parens.matching() && token.kind != TokenKind::OpenParen {
                return Err(Error::NoEnclosingParens);
            }

            match token.kind {
                TokenKind::OpenParen => parens.count += 1,
                TokenKind::CloseParen => parens.count -= 1,
                // TODO: Fill this out with parsing of other things.
                _ => {}
            }
        }

        if parens.not_matching() {
            Err(Error::UnmatchedParens)
        } else {
            match ast {
                Some(a) => Ok(a),
                None => Err(Error::EmptyProgram),
            }
        }
    }
}
