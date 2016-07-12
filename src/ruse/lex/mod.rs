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
    use super::lex::Lexer;
    use super::token::Token;

    #[test]
    fn lex_the_empty_program() {
        let tokens = Lexer::lex("").unwrap();
        let expected = vec![];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn lex_a_single_open_paren() {
        let tokens = Lexer::lex("(").unwrap();
        // Input stream locations are 1-indexed.
        let expected = vec![Token::open_paren(1)];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn lex_a_single_close_paren() {
        let tokens = Lexer::lex(")").unwrap();
        let expected = vec![Token::close_paren(1)];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn lex_matching_parens() {
        let tokens = Lexer::lex("()").unwrap();
        let expected = vec![Token::open_paren(1), Token::close_paren(2)];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn lex_a_simple_program() {
        let tokens = Lexer::lex("(+ 2 3)").unwrap();
        let expected = vec![
            Token::open_paren(1),
            Token::ident("+".to_string(), 2),
            Token::integer(2, 1, 4),
            Token::integer(3, 1, 6),
            Token::close_paren(7),
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn lex_a_more_complex_program() {
        let tokens = Lexer::lex("(+ (add-two 2) 3)").unwrap();
        let expected = vec![
            Token::open_paren(1),
            Token::ident("+".to_string(), 2),
            Token::open_paren(4),
            Token::ident("add-two".to_string(), 5),
            Token::integer(2, 1, 7),
            Token::close_paren(8),
            Token::integer(3, 1, 10),
            Token::close_paren(11),
        ];
    }
}
