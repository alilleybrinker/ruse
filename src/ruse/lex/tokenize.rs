//! An iterator for parsing tokens from an input stream.

use lex::{self, Token};
use std::cell::Cell;
use std::iter::Peekable;
use std::str::Chars;
use std::iter::Iterator;

/// An iterator over the tokens in an input string.
pub struct Tokenize<'a> {
    /// A peekable iterator over the characters in the original string.
    char_iter: Peekable<Chars<'a>>,
    /// The location of the iterator in the input stream.
    location: Cell<usize>,
}

impl<'a> Tokenize<'a> {
    /// Create a new Tokenize to iterate over the given string.
    pub fn new<'b>(s: &'b str) -> Tokenize<'b> {
        Tokenize {
            char_iter: s.chars().peekable(),
            location: Cell::new(0),
        }
    }

    /// Increment the iterator's internal location field.
    fn increment_location(&self) {
        let old_location = self.location.get();
        self.location.set(old_location + 1);
    }

    /// Parse an open parenthese.
    fn parse_open_paren(&self) -> Result<Token, lex::Error> {
        Ok(Token::open_paren(self.location.get()))
    }

    /// Parse a closed parenthese.
    fn parse_close_paren(&self) -> Result<Token, lex::Error> {
        Ok(Token::close_paren(self.location.get()))
    }

    /// Parse a number, either floating point or integer.
    fn parse_number(&mut self, character: char) -> Result<Token, lex::Error> {
        let mut len = 1;
        let mut result = Vec::new();
        result.push(character);

        while let Some(&next_character) = self.char_iter.peek() {
            match next_character {
                '0'...'9' => {
                    self.increment_location();
                    len += 1;
                    result.push(next_character);
                    self.char_iter.next();
                }
                '.' => {
                    self.increment_location();
                    len += 1;
                    result.push(next_character);
                    self.char_iter.next();
                }
                _ => break,
            }
        }

        let out: String = result.iter().cloned().collect();

        if let Ok(val) = out.parse::<i64>() {
            return Ok(Token::integer(val, len, self.location.get()));
        }

        if let Ok(val) = out.parse::<f64>() {
            return Ok(Token::float(val, len, self.location.get()));
        }

        Err(lex::Error::MalformedNumber(out))
    }

    /// Parse an identifier.
    fn parse_identifier(&mut self, character: char) -> Result<Token, lex::Error> {
        let mut result = Vec::new();
        result.push(character);

        while let Some(&next_character) = self.char_iter.peek() {
            match next_character {
                'a'...'z' | 'A'...'Z' | '!' | '$' | '%' | '&' | '*' | '/' | ':' | '<' | '=' |
                '>' | '?' | '^' | '_' | '~' | '0'...'9' | '+' | '-' | '.' | '@' => {
                    self.increment_location();

                    result.push(next_character);
                    self.char_iter.next();
                }
                ' ' | '\n' | '\t' | '\r' => break,
                _ => return Err(lex::Error::InvalidCharacter(next_character, self.location.get())),
            }
        }

        let out: String = result.iter().cloned().collect();
        Ok(Token::ident(out, self.location.get()))
    }
}

impl<'a> Iterator for Tokenize<'a> {
    /// Returns either a Token or a lexing error.
    type Item = Result<Token, lex::Error>;

    /// Returns one of three things:
    ///
    /// 1. `Option::None`
    /// 2. `Option::Some(Result::Err(lex::Error))`
    /// 3. `Option::Some(Result::Ok(Token))`
    ///
    /// Option (1) indicates that there's nothing left to parse. Option (2)
    /// indicates an error in the input stream. Option (3) is how the parsed
    /// tokens are returned to the user.
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(character) = self.char_iter.next() {
            self.increment_location();

            match character {
                '(' => return Some(self.parse_open_paren()),
                ')' => return Some(self.parse_close_paren()),
                '0'...'9' => return Some(self.parse_number(character)),
                'a'...'z' | 'A'...'Z' | '!' | '$' | '%' | '&' | '*' | '/' | ':' | '<' | '=' |
                '>' | '?' | '^' | '_' | '~' | '+' | '-' => {
                    return Some(self.parse_identifier(character))
                }
                ' ' | '\n' | '\t' | '\r' => (),
                _ => unreachable!(),
            }
        }

        None
    }
}

/// Extend a stringy type with the ability to generate tokens.
pub trait IterExt: AsRef<str> {
    /// Convenience method to get a token iterator for a string.
    fn tokens<'a>(&'a self) -> Tokenize<'a> {
        Tokenize::new(self.as_ref())
    }
}

impl IterExt for str {}
