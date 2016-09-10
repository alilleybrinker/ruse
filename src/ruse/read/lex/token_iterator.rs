//! An iterator for parsing tokens from an input stream.

use read::lex::{self, Token, Location, Error};
use std::cell::Cell;
use std::iter::Peekable;
use std::str::Chars;
use std::iter::Iterator;

/// An iterator over the tokens in an input string.
pub struct TokenIterator<'a> {
    /// A peekable iterator over the characters in the original string.
    char_iter: Peekable<Chars<'a>>,
    /// The location of the iterator in the input stream.
    location: Cell<Location>,
}

/// Indicates whether to move the internal character iterator.
enum Iterate {
    Yes,
    No,
}

impl<'a> TokenIterator<'a> {
    /// Create a new TokenIterator to iterate over the given string.
    pub fn new<'b>(s: &'b str) -> TokenIterator<'b> {
        TokenIterator {
            char_iter: s.chars().peekable(),
            location: Cell::new(Location(0)),
        }
    }

    /// Increment the iterator's internal location field.
    fn next_loc(&mut self, go_to_next: Iterate) {
        self.location.set(Location(self.is_at().0 + 1));

        if let Iterate::Yes = go_to_next {
            self.char_iter.next();
        }
    }

    /// Get the current location of the parser in the input text.
    fn is_at(&self) -> Location {
        self.location.get()
    }

    /// Parse an open parenthese.
    fn parse_open_paren(&self) -> Result<Token, lex::Error> {
        Ok(Token::open_paren(self.is_at()))
    }

    /// Parse a closed parenthese.
    fn parse_close_paren(&self) -> Result<Token, lex::Error> {
        Ok(Token::close_paren(self.is_at()))
    }

    /// Parse a number, either floating point or integer.
    ///
    /// Attempts to parse a number. For now, it parses only integers and
    /// floating point numbers. Eventually we'll want to cover the entire
    /// numeric tower that Scheme requires, which will demand substantially
    /// more work than is being done now.
    fn parse_number(&mut self, character: char) -> Result<Token, lex::Error> {
        let mut result = vec![character];
        let start = self.is_at();

        while let Some(&next_character) = self.char_iter.peek() {
            match next_character {
                '0'...'9' => {
                    self.next_loc(Iterate::Yes);
                    result.push(next_character);
                }
                '.' => {
                    self.next_loc(Iterate::Yes);
                    result.push(next_character);
                }
                _ => break,
            }
        }

        let out: String = result.iter().cloned().collect();
        let end = self.is_at();

        if let Ok(val) = out.parse::<i64>() {
            Ok(Token::integer(val, start, end))
        } else if let Ok(val) = out.parse::<f64>() {
            Ok(Token::float(val, start, end))
        } else {
            Err(Error::MalformedNumber(out))
        }
    }

    /// Parse an identifier.
    ///
    /// This parses an identifier, starting with the given character, which is
    /// passed in for convenience. Note that at the moment the character
    /// matching rules are kept in sync manually between this function and the
    /// original dispatcher, and that they are not exactly the same. There are
    /// certain characters which are acceptible within an identifier that are
    /// not acceptable at the start of one.
    fn parse_identifier(&mut self, character: char) -> Result<Token, lex::Error> {
        let mut result = vec![character];
        let start = self.is_at();

        while let Some(&next_character) = self.char_iter.peek() {
            match next_character {
                'a'...'z' | 'A'...'Z' | '!' | '$' | '%' | '&' | '*' | '/' | ':' | '<' | '=' |
                '>' | '?' | '^' | '_' | '~' | '0'...'9' | '+' | '-' | '.' | '@' => {
                    self.next_loc(Iterate::Yes);
                    result.push(next_character);
                }
                // Stop on whitespace.
                ' ' | '\n' | '\t' | '\r' => break,
                _ => return Err(Error::InvalidCharacter(next_character, start.0)),
            }
        }

        let out: String = result.iter().cloned().collect();
        Ok(Token::ident(out, start))
    }
}

impl<'a> Iterator for TokenIterator<'a> {
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
            self.next_loc(Iterate::No);

            match character {
                '(' => return Some(self.parse_open_paren()),
                ')' => return Some(self.parse_close_paren()),
                '0'...'9' => return Some(self.parse_number(character)),
                'a'...'z' | 'A'...'Z' | '!' | '$' | '%' | '&' | '*' | '/' | ':' | '<' | '=' |
                '>' | '?' | '^' | '_' | '~' | '+' | '-' => {
                    return Some(self.parse_identifier(character))
                }
                // Skip whitespace.
                ' ' | '\n' | '\t' | '\r' => (),
                _ => return Some(Err(Error::InvalidCharacter(character, self.is_at().0))),
            }
        }

        None
    }
}

/// Extend a stringy type with the ability to generate tokens.
pub trait StrTokenIterator: AsRef<str> {
    /// Convenience method to get a token iterator for a string.
    fn tokens<'a>(&'a self) -> TokenIterator<'a> {
        TokenIterator::new(self.as_ref())
    }
}

// Implement StrIterExt for all types that can be ref'ed into string slices.
impl<T: AsRef<str>> StrTokenIterator for T {}
