use parse::{LexResult, TokenIterator};
use std::cell::Cell;

pub struct Lexer<'a> {
    location: Cell<usize>,
    token_iter: TokenIterator<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(s: &str) -> Lexer {
        Lexer {
            location: Cell::new(0),
            token_iter: TokenIterator::new(s),
        }
    }

    pub fn lex(&self) -> LexResult {
        // Construct a Vec<Token<'a>> from the TokenIterator, erroring out if a
        // lexing error is encountered.
        //
        // self.location is used to track where we are in the input stream.
        unimplemented!();
    }
}
