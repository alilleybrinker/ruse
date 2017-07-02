//! Get a vector of tokens from an input string.

pub mod error;
pub mod token;

use lex::error::Result;
use lex::token::StrTokenIterator;

/// Lexes an input string to get a vector of tokens from it.
pub trait Lex: AsRef<str> {
    /// Get a vector of tokens from the given string, or a lex::Error if there's
    /// something wrong with the input stream.
    fn lex(&self) -> Result {
        self.as_ref().tokens().collect::<Result>()
    }
}

impl<T: AsRef<str>> Lex for T {}

#[cfg(test)]
mod tests {
    use lex::Lex;
    use lex::token::{Token, Location};
    use lex::error::Error;

    #[test]
    fn lex_the_empty_program() {
        let tokens = "".lex();
        let expected = Ok(vec![]);
        assert_eq!(tokens, expected);
    }

    #[test]
    fn lex_a_single_open_paren() {
        let tokens = "(".lex();
        let expected = Ok(vec![Token::open_paren(Location(1))]);
        assert_eq!(tokens, expected);
    }

    #[test]
    fn lex_a_single_close_paren() {
        let tokens = ")".lex();
        let expected = Ok(vec![Token::close_paren(Location(1))]);
        assert_eq!(tokens, expected);
    }

    #[test]
    fn lex_matching_parens() {
        let tokens = "()".lex();
        let expected = Ok(vec![
            Token::open_paren(Location(1)),
            Token::close_paren(Location(2)),
        ]);
        assert_eq!(tokens, expected);
    }

    #[test]
    fn lex_a_simple_program() {
        let tokens = "(+ 2 3)".lex();
        let expected = Ok(vec![
            Token::open_paren(Location(1)),
            Token::ident("+", Location(2)),
            Token::integer(2, Location(4), Location(4)),
            Token::integer(3, Location(6), Location(6)),
            Token::close_paren(Location(7)),
        ]);
        assert_eq!(tokens, expected);
    }

    #[test]
    fn lex_a_more_complex_program() {
        let tokens = "(+ (add-two 2) 3.2)".lex();
        let expected = Ok(vec![
            Token::open_paren(Location(1)),
            Token::ident("+", Location(2)),
            Token::open_paren(Location(4)),
            Token::ident("add-two", Location(5)),
            Token::integer(2, Location(13), Location(13)),
            Token::close_paren(Location(14)),
            Token::float(3.2, Location(16), Location(18)),
            Token::close_paren(Location(19)),
        ]);
        assert_eq!(tokens, expected);
    }

    #[test]
    fn lex_a_complex_identifier() {
        let tokens = "(%a+/d 2 4)".lex();
        let expected = Ok(vec![
            Token::open_paren(Location(1)),
            Token::ident("%a+/d", Location(2)),
            Token::integer(2, Location(8), Location(8)),
            Token::integer(4, Location(10), Location(10)),
            Token::close_paren(Location(11)),
        ]);
        assert_eq!(tokens, expected);
    }

    #[test]
    fn fail_to_lex_a_non_ascii_character() {
        let result = "(+ (¢ 3) 4)".lex();
        let expected = Err(Error::InvalidCharacter('¢', 5));
        assert_eq!(result, expected);
    }

    #[test]
    fn lex_a_short_boolean() {
        let result = "(#t)".lex();
        let expected = Ok(vec![
            Token::open_paren(Location(1)),
            Token::boolean(true, Location(2), Location(3)),
            Token::close_paren(Location(4)),
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn lex_a_long_boolean() {
        let result = "(#false)".lex();
        let expected = Ok(vec![
            Token::open_paren(Location(1)),
            Token::boolean(false, Location(2), Location(7)),
            Token::close_paren(Location(8)),
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn lex_an_invalid_literal() {
        let result = "(#what)".lex();
        let expected = Err(Error::InvalidLiteral("#what".to_string(), 2));
        assert_eq!(result, expected);
    }

    #[test]
    fn lex_a_string() {
        let result = "\"hello\"".lex();
        let expected = Ok(vec![Token::string("hello".to_string(), Location(1))]);
        assert_eq!(result, expected);
    }

    #[test]
    fn lex_a_string_in_a_function() {
        let result = "(f 2 \"blah\")".lex();
        let expected = Ok(vec![
            Token::open_paren(Location(1)),
            Token::ident("f", Location(2)),
            Token::integer(2, Location(4), Location(4)),
            Token::string("blah", Location(6)),
            Token::close_paren(Location(12)),
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn lex_a_string_with_an_escape_equence() {
        let result = "(g \"hello\n\" 4)".lex();
        let expected = Ok(vec![
            Token::open_paren(Location(1)),
            Token::ident("g", Location(2)),
            Token::string("hello\n", Location(4)),
            Token::integer(4, Location(13), Location(13)),
            Token::close_paren(Location(14)),
        ]);
        assert_eq!(result, expected);
    }
}
