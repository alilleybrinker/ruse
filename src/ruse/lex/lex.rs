use lex::{self, IterExt};

/// Lexes an input string to get a vector of tokens from it.
pub struct Lexer;

impl Lexer {
    /// Get a vector of tokens from the given string, or a lex::Error if there's
    /// something wrong with the input stream.
    pub fn lex(s: &str) -> lex::Result {
        s.tokens().collect::<lex::Result>()
    }
}
