use parse::{LexResult, TokenIterator};
use std::cell::Cell;

pub struct Lexer {
    location: Cell<usize>,
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer { location: Cell::new(0) }
    }

    pub fn lex<'a>(&self, s: TokenIterator<'a>) -> LexResult {
        // Construct a Vec<Token<'a>> from the TokenIterator, erroring out if a
        // lexing error is encountered.
        //
        // self.location is used to track where we are in the input stream.
        unimplemented!();
    }
}
