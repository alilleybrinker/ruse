//! Get a vector of tokens from an input string.
// Note that this module is mostly tests. This is by
// design. The parts that the user should care about
// are the lexer interface, the error type, and the
// result type. Everything else is really an implementation
// detail.

pub mod error;
pub mod token;

pub use lex::error::{Error, Result};
use lex::token::StrTokenIterator;

/// Lexes an input string to get a vector of tokens from it.
pub fn lex<S: AsRef<str>>(s: S) -> Result {
    // Note that the `collect` call does some magic thanks to
    // a stdlib impl for `Vec`. It actually auto-converts a
    // `Vec<Result<Token, Error>>` into `Result<Vec<Token>, Error>`.
    // This means that only the first error is retained, which
    // for now is what we want to do. In the future we'd likely
    // want to have a better strategy which doesn't halt lexing
    // at every error, and instead differentiates between errors
    // which can be worked around and errors which can't, or at
    // least tries to provide more information for the user to
    // address at once, rather than requiring a slower feedback
    // cycle, which can be annoying.
    s.as_ref().tokens().collect::<Result>()
}

#[cfg(test)]
mod tests {
    use lex::lex;
    use lex::Error;
    use lex::token::{Token, Location};

    #[test]
    fn lex_the_empty_program() {
        let tokens = lex("");
        let expected = Ok(vec![]);
        assert_eq!(tokens, expected);
    }

    #[test]
    fn lex_a_single_open_paren() {
        let tokens = lex("(");
        let expected = Ok(vec![Token::open_paren(Location(1))]);
        assert_eq!(tokens, expected);
    }

    #[test]
    fn lex_a_single_close_paren() {
        let tokens = lex(")");
        let expected = Ok(vec![Token::close_paren(Location(1))]);
        assert_eq!(tokens, expected);
    }

    #[test]
    fn lex_matching_parens() {
        let tokens = lex("()");
        let expected = Ok(vec![
            Token::open_paren(Location(1)),
            Token::close_paren(Location(2)),
        ]);
        assert_eq!(tokens, expected);
    }

    #[test]
    fn lex_a_simple_program() {
        let tokens = lex("(+ 2 3)");
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
        let tokens = lex("(+ (add-two 2) 3.2)");
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
        let tokens = lex("(%a+/d 2 4)");
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
        let result = lex("(+ (¢ 3) 4)");
        let expected = Err(Error::InvalidCharacter('¢', 5));
        assert_eq!(result, expected);
    }

    #[test]
    fn lex_a_short_boolean() {
        let result = lex("(#t)");
        let expected = Ok(vec![
            Token::open_paren(Location(1)),
            Token::boolean(true, Location(2), Location(3)),
            Token::close_paren(Location(4)),
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn lex_a_long_boolean() {
        let result = lex("(#false)");
        let expected = Ok(vec![
            Token::open_paren(Location(1)),
            Token::boolean(false, Location(2), Location(7)),
            Token::close_paren(Location(8)),
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn lex_an_invalid_literal() {
        let result = lex("(#what)");
        let expected = Err(Error::InvalidLiteral("#what".to_string(), 2));
        assert_eq!(result, expected);
    }

    #[test]
    fn lex_a_string() {
        let result = lex("\"hello\"");
        let expected = Ok(vec![Token::string("hello".to_string(), Location(1))]);
        assert_eq!(result, expected);
    }

    #[test]
    fn lex_a_string_in_a_function() {
        let result = lex("(f 2 \"blah\")");
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
        let result = lex("(g \"hello\n\" 4)");
        let expected = Ok(vec![
            Token::open_paren(Location(1)),
            Token::ident("g", Location(2)),
            Token::string("hello\n", Location(4)),
            Token::integer(4, Location(13), Location(13)),
            Token::close_paren(Location(14)),
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn lex_mix_of_delimiters() {
        let result = lex("([{}])");
        let expected = Ok(vec![
            Token::open_paren(Location(1)),
            Token::open_bracket(Location(2)),
            Token::open_brace(Location(3)),
            Token::close_brace(Location(4)),
            Token::close_bracket(Location(5)),
            Token::close_paren(Location(6)),
        ]);
        assert_eq!(result, expected);
    }
}
