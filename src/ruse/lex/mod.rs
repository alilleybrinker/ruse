//! Get a vector of tokens from an input string.

pub mod error;
pub mod lex;
pub mod token;
pub mod tokenize;

pub use self::error::*;
pub use self::lex::*;
pub use self::token::*;
pub use self::tokenize::*;

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn lex_the_empty_program() {
        assert_eq!(4, 4);
    }

    #[test]
    fn lex_a_single_open_paren() {
        assert_eq!(4, 4);
    }

    #[test]
    fn lex_a_single_close_paren() {
        assert_eq!(4, 4);
    }

    #[test]
    fn lex_matching_parens() {
        assert_eq!(4, 4);
    }

    #[test]
    fn lex_a_simple_program() {
        assert_eq!(4, 4);
    }

    #[test]
    fn lex_a_more_complex_program() {
        assert_eq!(4, 4);
    }
}
