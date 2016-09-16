//! Generate a syntax tree from an input stream.

pub mod error;
pub mod expr;

use read::lex::token::Token;
use read::parse::error::Result;

/// Parse an input string, returning a syntax tree that can be evaluated.
pub trait Parse {
    /// This function takes in a string representing a ruse expression, and
    /// parses it into an abstract syntax tree.
    ///
    /// The SyntaxTree is a tree of SyntaxNodes, each of which contains the
    /// original text and some data associated with the text.
    ///
    /// For example, the following program:
    ///
    /// ```ruse
    /// (+ 2 3)
    /// ```
    ///
    /// Becomes:
    ///
    /// ```text
    /// [<open_paren>, <ident:+>, <int_literal: 2>, <int_literal: 3>, <close_paren>]
    /// ```
    ///
    /// Becomes:
    ///
    /// ```text
    /// Expr::List(Vec(Expr::Str("+"), Expr::Integer(2), Expr::Integer(3)))
    /// ```
    fn parse(&self) -> Result {
        unimplemented!()
    }
}

impl Parse for Vec<Token> {}
