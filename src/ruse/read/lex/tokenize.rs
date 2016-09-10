//! An iterator for parsing tokens from an input stream.

use read::lex::{self, Token, Location, Error};
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
        Ok(Token::open_paren(Location(self.location.get())))
    }

    /// Parse a closed parenthese.
    fn parse_close_paren(&self) -> Result<Token, lex::Error> {
        Ok(Token::close_paren(Location(self.location.get())))
    }

    /// Parse a number, either floating point or integer.
    ///
    /// Attempts to parse a number. For now, it parses only integers and
    /// floating point numbers. Eventually we'll want to cover the entire
    /// numeric tower that Scheme requires, which will demand substantially
    /// more work than is being done now.
    fn parse_number(&mut self, character: char) -> Result<Token, lex::Error> {
        let mut result = vec![character];
        let location = self.location.get();

        while let Some(&next_character) = self.char_iter.peek() {
            match next_character {
                '0'...'9' => {
                    self.increment_location();
                    result.push(next_character);
                    self.char_iter.next();
                }
                '.' => {
                    self.increment_location();
                    result.push(next_character);
                    self.char_iter.next();
                }
                _ => break,
            }
        }

        let out: String = result.iter().cloned().collect();

        let start_loc = Location(location);
        let end_loc = Location(self.location.get());

        if let Ok(val) = out.parse::<i64>() {
            return Ok(Token::integer(val, start_loc, end_loc));
        }

        if let Ok(val) = out.parse::<f64>() {
            return Ok(Token::float(val, start_loc, end_loc));
        }

        Err(Error::MalformedNumber(out))
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
        let location = self.location.get();

        while let Some(&next_character) = self.char_iter.peek() {
            match next_character {
                'a'...'z' | 'A'...'Z' | '!' | '$' | '%' | '&' | '*' | '/' | ':' | '<' | '=' |
                '>' | '?' | '^' | '_' | '~' | '0'...'9' | '+' | '-' | '.' | '@' => {
                    self.increment_location();

                    result.push(next_character);
                    self.char_iter.next();
                }
                // Stop on whitespace.
                ' ' | '\n' | '\t' | '\r' => break,
                _ => return Err(Error::InvalidCharacter(next_character, location)),
            }
        }

        let out: String = result.iter().cloned().collect();
        Ok(Token::ident(out, Location(location)))
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
                // Skip whitespace.
                ' ' | '\n' | '\t' | '\r' => (),
                _ => return Some(Err(Error::InvalidCharacter(character, self.location.get()))),
            }
        }

        None
    }
}

/// Extend a stringy type with the ability to generate tokens.
pub trait StrIterExt: AsRef<str> {
    /// Convenience method to get a token iterator for a string.
    fn tokens<'a>(&'a self) -> Tokenize<'a> {
        Tokenize::new(self.as_ref())
    }
}

impl StrIterExt for str {}
