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

impl Parse for Vec<Token> {
    /// Parse a vector of tokens into an AST.
    fn parse(&self) -> Result {
        let ast: Option<_> = None;
        let mut has_parens = 0;

        for token in self {
            if has_parens == -1 {
                return Err(Error::NoEnclosingParens);
            }

            match token.kind {
                TokenKind::OpenParen => {
                    if has_parens == 0 && token.kind != TokenKind::OpenParen {
                        return Err(Error::NoEnclosingParens);
                    }

                    has_parens += 1;
                }
                TokenKind::CloseParen => {
                    has_parens -= 1;

                    if has_parens == 0 {
                        has_parens = -1;
                    }
                }
                // TODO: Fill this out with parsing of other things.
                _ => {}
            }
        }

        if has_parens != 0 {
            Err(Error::UnmatchedParens)
        } else {
            match ast {
                Some(a) => Ok(a),
                None => Err(Error::EmptyParens),
            }
        }
    }
}
