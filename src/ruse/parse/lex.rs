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
}

impl<'a> TokenIterator<'a> {
    pub fn new<'b>(s: &'b str) -> TokenIterator<'b> {
        TokenIterator { char_iter: s.chars().peekable() }
    }
}

impl<'a> Iterator for TokenIterator<'a> {
    type Item = Result<Token<'a>, LexError>;

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}
