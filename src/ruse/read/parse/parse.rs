//! Generate a syntax tree from an input stream.

use read::parse;
use read::lex::Token;

/// Parse an input string, returning a syntax tree that can be evaluated.
pub struct Parser;

impl Parser {
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
    ///         <fn: '+'>
    ///         /       \
    /// <atom: 2>       <atom: 3>
    /// ```
    ///
    /// For now, this is done by assuming that the first item after an open
    /// paren is a function call, and that everything after is an atom. This
    /// will obviously become better over time.
    pub fn parse(_: &[Token]) -> parse::Result {
        unimplemented!()
    }
}
