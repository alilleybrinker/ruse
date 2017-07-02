//! Generate a syntax tree from an input stream.

pub mod error;
pub mod expr;

pub use parse::error::{Error, Result};
use lex::token::{Token, TokenKind};
use parse::expr::Expr;

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
        Parens { count: 0 }
    }

    fn matching(&self) -> bool {
        self.count != 0
    }

    fn not_matching(&self) -> bool {
        !self.matching()
    }
}

impl Parse for Vec<Token> {
    /// Parse a vector of tokens into an AST.
    fn parse(&self) -> Result {
        let ast: Option<Expr> = None;
        let mut parens = Parens::new();

        for token in self {
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
}
