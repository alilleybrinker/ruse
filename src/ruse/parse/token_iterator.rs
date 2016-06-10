use std::iter::Peekable;
use std::str::Chars;

pub struct TokenIterator<'a> {
    char_iterator: Peekable<Chars<'a>>,
}

impl<'a> TokenIterator<'a> {
    pub fn new<'b>(s: &'b str) -> TokenIterator<'b> {
        TokenIterator { char_iterator: s.chars().peekable() }
    }
}

pub enum Token<'a> {
    Paren,
    Ident(&'a str),
}
