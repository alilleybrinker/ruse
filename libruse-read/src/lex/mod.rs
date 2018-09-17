pub mod error;
pub mod token;
pub use lex::error::{Error, Result};
use lex::token::StrTokenIterator;

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
        let expected =
            Ok(
                vec![
                    Token::open_paren(
                        Location::new(1, 1),
                        Location::new(1, 2)
                    )
                ]
            );
        assert_eq!(tokens, expected);
    }

    #[test]
    fn lex_a_single_close_paren() {
        let tokens = lex(")");
        let expected =
            Ok(
                vec![
                    Token::close_paren(
                        Location::new(1, 1),
                        Location::new(1, 2)
                    )
                ]
            );
        assert_eq!(tokens, expected);
    }

    #[test]
    fn lex_matching_parens() {
        let tokens = lex("()");
        let expected = Ok(vec![
            Token::open_paren(Location::new(1, 1), Location::new(1, 2)),
            Token::close_paren(Location::new(1, 2), Location::new(1, 3)),
        ]);
        assert_eq!(tokens, expected);
    }

    #[test]
    fn lex_a_simple_program() {
        let tokens = lex("(+ 2 3)");
        let expected = Ok(vec![
            Token::open_paren(Location::new(1, 1), Location::new(1, 2)),
            Token::symbol("+", Location::new(1, 2)),
            Token::integer(2, Location::new(1, 4), Location::new(1, 5)),
            Token::integer(3, Location::new(1, 6), Location::new(1, 7)),
            Token::close_paren(Location::new(1, 7), Location::new(1, 8)),
        ]);
        assert_eq!(tokens, expected);
    }

    #[test]
    fn lex_a_more_complex_program() {
        let tokens = lex("(+ (add-two 2) 3.2)");
        let expected = Ok(vec![
            Token::open_paren(Location::new(1, 1), Location::new(1, 2)),
            Token::symbol("+", Location::new(1, 2)),
            Token::open_paren(Location::new(1, 4), Location::new(1, 5)),
            Token::symbol("add-two", Location::new(1, 5)),
            Token::integer(2, Location::new(1, 13), Location::new(1, 14)),
            Token::close_paren(Location::new(1, 14), Location::new(1, 15)),
            Token::float(3.2, Location::new(1, 16), Location::new(1, 19)),
            Token::close_paren(Location::new(1, 19), Location::new(1, 20)),
        ]);
        assert_eq!(tokens, expected);
    }

    #[test]
    fn lex_a_complex_symbol() {
        let tokens = lex("(%a+/d 2 4)");
        let expected = Ok(vec![
            Token::open_paren(Location::new(1, 1), Location::new(1, 2)),
            Token::symbol("%a+/d", Location::new(1, 2)),
            Token::integer(2, Location::new(1, 8), Location::new(1, 9)),
            Token::integer(4, Location::new(1, 10), Location::new(1, 11)),
            Token::close_paren(Location::new(1, 11), Location::new(1, 12)),
        ]);
        assert_eq!(tokens, expected);
    }

    #[test]
    fn fail_to_lex_a_non_ascii_character() {
        let result = lex("(+ (¢ 3) 4)");
        let expected = Err(Error::InvalidCharacter('¢', Location::new(1, 5)));
        assert_eq!(result, expected);
    }

    #[test]
    fn lex_a_short_boolean() {
        let result = lex("(#t)");
        let expected = Ok(vec![
            Token::open_paren(Location::new(1, 1), Location::new(1, 2)),
            Token::boolean(true, Location::new(1, 2), Location::new(1, 4)),
            Token::close_paren(Location::new(1, 4), Location::new(1, 5)),
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn lex_a_long_boolean() {
        let result = lex("(#false)");
        let expected = Ok(vec![
            Token::open_paren(Location::new(1, 1), Location::new(1, 2)),
            Token::boolean(false, Location::new(1, 2), Location::new(1, 8)),
            Token::close_paren(Location::new(1, 8), Location::new(1, 9)),
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn lex_an_invalid_literal() {
        let result = lex("(#what)");
        let expected = Err(Error::InvalidLiteral("#what".to_string(), Location::new(1, 2)));
        assert_eq!(result, expected);
    }

    #[test]
    fn lex_a_string() {
        let result = lex("\"hello\"");
        let expected =
            Ok(
                vec![
                    Token::string(
                        "hello".to_string(),
                        Location::new(1, 1)
                    )
                ]
            );
        assert_eq!(result, expected);
    }

    #[test]
    fn lex_a_string_in_a_function() {
        let result = lex("(f 2 \"blah\")");
        let expected = Ok(vec![
            Token::open_paren(Location::new(1, 1), Location::new(1, 2)),
            Token::symbol("f", Location::new(1, 2)),
            Token::integer(2, Location::new(1, 4), Location::new(1, 5)),
            Token::string("blah", Location::new(1, 6)),
            Token::close_paren(Location::new(1, 12), Location::new(1, 13)),
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn lex_a_string_with_an_escape_equence() {
        let result = lex("(g \"hello\n\" 4)");
        let expected = Ok(vec![
            Token::open_paren(Location::new(1, 1), Location::new(1, 2)),
            Token::symbol("g", Location::new(1, 2)),
            Token::string("hello\n", Location::new(1, 4)),
            Token::integer(4, Location::new(1, 13), Location::new(1, 14)),
            Token::close_paren(Location::new(1, 14), Location::new(1, 15)),
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn lex_mix_of_delimiters() {
        let result = lex("([{}])");
        let expected = Ok(vec![
            Token::open_paren(Location::new(1, 1), Location::new(1, 2)),
            Token::open_bracket(Location::new(1, 2), Location::new(1, 3)),
            Token::open_brace(Location::new(1, 3), Location::new(1, 4)),
            Token::close_brace(Location::new(1, 4), Location::new(1, 5)),
            Token::close_bracket(Location::new(1, 5), Location::new(1, 6)),
            Token::close_paren(Location::new(1, 6), Location::new(1, 7)),
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn lex_multiline_program() {
        let program = "(+ 1 2)
(+ 3 4)";
        let result = lex(program);
        let expected = Ok(vec![
            Token::open_paren(Location::new(1, 1), Location::new(1, 2)),
            Token::symbol("+", Location::new(1, 2)),
            Token::integer(1, Location::new(1, 4), Location::new(1, 5)),
            Token::integer(2, Location::new(1, 6), Location::new(1, 7)),
            Token::close_paren(Location::new(1, 7), Location::new(1,8)),

            Token::open_paren(Location::new(2, 1), Location::new(2, 2)),
            Token::symbol("+", Location::new(2, 2)),
            Token::integer(1, Location::new(2, 4), Location::new(2, 5)),
            Token::integer(2, Location::new(2, 6), Location::new(2, 7)),
            Token::close_paren(Location::new(2, 7), Location::new(2, 8))
        ]);
        assert_eq!(result, expected);
    }
}

