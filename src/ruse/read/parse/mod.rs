//! Generate a syntax tree from an input stream.

pub mod error;
pub mod expr;

use read::lex::token::Token;
use read::parse::error::Result;

/// Parse an input string, returning a Ruse expression that can be evaluated.
pub trait Parse {
    /// Parse a type into an AST.
    fn parse(&self) -> Result;
}

impl Parse for Vec<Token> {
    /// Parse a vector of tokens into an AST.
    fn parse(&self) -> Result {
        unimplemented!()
    }
}
