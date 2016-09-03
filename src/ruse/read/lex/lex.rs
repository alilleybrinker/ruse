//! Generate a vector of tokens from an input stream.

use read::lex::{self, IterExt};

/// Lexes an input string to get a vector of tokens from it.
pub struct Lexer;

impl Lexer {
    /// Get a vector of tokens from the given string, or a lex::Error if there's
    /// something wrong with the input stream.
    pub fn lex<S: AsRef<str>>(s: S) -> lex::Result {
        s.as_ref().tokens().collect::<lex::Result>()
    }
}
