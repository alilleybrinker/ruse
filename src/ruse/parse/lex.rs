use parse::{LexResult, LexError, Token};
use std::cell::Cell;
use std::iter::Peekable;
use std::str::Chars;

pub struct Lexer {
    location: Cell<usize>,
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer { location: Cell::new(0) }
    }

    pub fn lex<'a>(&'a self, s: &'a str) -> LexResult {
        let token_iter = TokenIterator::new(s);
        token_iter.inspect(|t: &Result<Token, LexError>| {
                      if let Ok(ref token) = *t {
                          let old_location = self.location.get();
                          self.location.set(old_location + token.span);
                      };
                  })
                  .collect::<LexResult>()
    }
}

pub struct TokenIterator<'a> {
    char_iter: Peekable<Chars<'a>>,
    location: Cell<usize>,
}

pub type TokenResult<'a> = Result<Token<'a>, LexError>;

impl<'a> TokenIterator<'a> {
    pub fn new<'b>(s: &'b str) -> TokenIterator<'b> {
        TokenIterator {
            char_iter: s.chars().peekable(),
            location: Cell::new(0),
        }
    }

    fn parse_ident(&mut self) -> TokenResult {
        unimplemented!();
    }

    fn parse_int_literal(&mut self) -> TokenResult {
        unimplemented!();
    }

    fn parse_float_literal(&mut self) -> TokenResult {
        unimplemented!();
    }
}

impl<'a> Iterator for TokenIterator<'a> {
    type Item = TokenResult<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(character) = self.char_iter.next() {
            let old_location = self.location.get();
            self.location.set(old_location + 1);
            match character {
                '(' => return Some(Ok(Token::open_paren(self.location.get()))),
                ')' => return Some(Ok(Token::close_paren(self.location.get()))),
                // Also want to parse identifiers, ints, and floats.
                _ => unimplemented!(),
            }
        }
        unimplemented!();
    }
}
