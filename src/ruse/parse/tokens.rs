use std::iter::Peekable;
use std::str::Chars;

pub struct TokenIterator<'a> {
    char_iter: Peekable<Chars<'a>>,
}

impl<'a> TokenIterator<'a> {
    pub fn new<'b>(s: &'b str) -> TokenIterator<'b> {
        TokenIterator { char_iter: s.chars().peekable() }
    }
}

impl<'a> Iterator for TokenIterator<'a> {
    type Item = Token<'a>;

    pub fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}

pub struct Token<'a> {
    pub kind: TokenKind<'a>,
    pub location: usize,
    pub span: usize,
}

impl<'a> Token<'a> {
    pub fn paren(location: usize) -> Token<'a> {
        Token {
            kind: TokenKind::Paren,
            location: location,
            span: 1,
        }
    }

    pub fn ident(name: &'a str, location: usize) -> Token<'a> {
        Token {
            kind: TokenKind::Ident(name),
            location: location,
            span: name.len(),
        }
    }
}

pub enum TokenKind<'a> {
    Paren,
    Ident(&'a str),
}
