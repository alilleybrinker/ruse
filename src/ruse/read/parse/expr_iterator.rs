//! An iterator for parsing an expression from a stream of tokens.
//! 用于从标记流解析表达式的迭代器。

use read::parse::error::Error;
use read::parse::expr::Expr;
use read::lex::token::Token;

use std::cell::Cell;
use std::iter::Peekable;
use std::str::Chars;
use std::iter::Iterator;

/// An iterator over the tokens in an input string.
pub struct ExprIterator<'a> {
    /// A peekable iterator over the characters in the original string.
    token_iter: Peekable<Iter<'a>>,
}

impl<'a> ExprIterator<'a> {
    /// Create a new ExprIterator to iterate over the given string.
    pub fn new(s: &[Token]) -> ExprIterator {
        ExprIterator {
            char_iter: s.iter().peekable(),
        }
    }
}

impl<'a> Iterator for ExprIterator<'a> {
    /// Returns either an Expr or a parsing error.
    type Item = Result<Expr, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(character) = self.token_iter.next() {
			unimplemented!();
        }

        None
    }
}

