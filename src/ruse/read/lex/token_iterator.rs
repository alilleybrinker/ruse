//! An iterator for parsing tokens from an input stream.

use read::lex::error::Error;
use read::lex::token::{Token, Location};

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
enum IterateInternally {
    Yes,
    No,
}

impl<'a> TokenIterator<'a> {
    /// Create a new TokenIterator to iterate over the given string.
    pub fn new(s: &str) -> TokenIterator {
        TokenIterator {
            char_iter: s.chars().peekable(),
            location: Cell::new(Location(0)),
        }
    }

    /// Increment the iterator's internal location field.
    fn next_location(&mut self, go_to_next: IterateInternally) {
        self.location.set(Location(self.location().0 + 1));

        if let IterateInternally::Yes = go_to_next {
            self.char_iter.next();
        }
    }

    /// Get the current location of the parser in the input text.
    fn location(&self) -> Location {
        self.location.get()
    }
}

impl<'a> Iterator for TokenIterator<'a> {
    /// Returns either a Token or a lexing error.
    type Item = Result<Token, Error>;

    /// Returns one of three things:
    ///
    /// 1. `Option::None`
    /// 2. `Option::Some(Result::Err(Error))`
    /// 3. `Option::Some(Result::Ok(Token))`
    ///
    /// Option (1) indicates that there's nothing left to parse. Option (2)
    /// indicates an error in the input stream. Option (3) is how the parsed
    /// tokens are returned to the user.
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(character) = self.char_iter.next() {
            self.next_location(IterateInternally::No);

            // Decide what to attempt to lex based on the first character. Note that the
            // allowable characters for identifiers are fewer here then they are in the
            // lex_atom function.
            match character {
                '(' => return Some(lex_open_paren(self)),
                ')' => return Some(lex_close_paren(self)),
                '#' => return Some(lex_boolean(self, character)),
                '0'...'9' => return Some(lex_number(self, character)),
                'a'...'z' | 'A'...'Z' | '!' | '$' | '%' | '&' | '*' | '/' | ':' | '<' | '=' |
                '>' | '?' | '^' | '_' | '~' | '+' | '-' => return Some(lex_atom(self, character)),
                '"' => return Some(lex_string(self, character)),
                // Skip whitespace.
                ' ' | '\n' | '\t' | '\r' => (),
                _ => return Some(Err(Error::InvalidCharacter(character, self.location().0))),
            }
        }

        None
    }
}

/// Extend a stringy type with the ability to generate tokens.
pub trait StrTokenIterator: AsRef<str> {
    /// Convenience method to get a token iterator for a string.
    fn tokens(&self) -> TokenIterator {
        TokenIterator::new(self.as_ref())
    }
}

// Implement StrIterExt for all types that can be ref'ed into string slices.
impl<T: AsRef<str>> StrTokenIterator for T {}

/// Parse an open parenthese.
fn lex_open_paren(iter: &TokenIterator) -> Result<Token, Error> {
    Ok(Token::open_paren(iter.location()))
}

/// Parse a closed parenthese.
fn lex_close_paren(iter: &TokenIterator) -> Result<Token, Error> {
    Ok(Token::close_paren(iter.location()))
}

/// Parse a number, either floating point or integer.
///
/// Attempts to parse a number. For now, it parses only integers and
/// floating point numbers. Eventually we'll want to cover the entire
/// numeric tower that Scheme requires, which will demand substantially
/// more work than is being done now.
fn lex_number(iter: &mut TokenIterator, character: char) -> Result<Token, Error> {
    let mut result = vec![character];
    let start = iter.location();

    while let Some(&next_character) = iter.char_iter.peek() {
        match next_character {
            '0'...'9' | '.' => {
                iter.next_location(IterateInternally::Yes);
                result.push(next_character);
            }
            _ => break,
        }
    }

    let out: String = result.iter().cloned().collect();
    let end = iter.location();

    if let Ok(val) = out.parse::<i64>() {
        Ok(Token::integer(val, start, end))
    } else if let Ok(val) = out.parse::<f64>() {
        Ok(Token::float(val, start, end))
    } else {
        Err(Error::MalformedNumber(out, start.0))
    }
}

/// Parse an atom.
///
/// This parses an atom, starting with the given character, which is
/// passed in for convenience. Note that at the moment the character
/// matching rules are kept in sync manually between this function and the
/// original dispatcher, and that they are not exactly the same. There are
/// certain characters which are acceptible within an identifier that are
/// not acceptable at the start of one.
fn lex_atom(iter: &mut TokenIterator, character: char) -> Result<Token, Error> {
    let mut result = vec![character];
    let start = iter.location();

    while let Some(&next_character) = iter.char_iter.peek() {
        match next_character {
            'a'...'z' | 'A'...'Z' | '!' | '$' | '%' | '&' | '*' | '/' | ':' | '<' | '=' | '>' |
            '?' | '^' | '_' | '~' | '0'...'9' | '+' | '-' | '.' | '@' => {
                iter.next_location(IterateInternally::Yes);
                result.push(next_character);
            }
            // Stop on whitespace or parentheses.
            '(' | ')' | ' ' | '\n' | '\t' | '\r' => break,
            _ => return Err(Error::InvalidCharacter(next_character, start.0)),
        }
    }

    let out: String = result.iter().cloned().collect();

    Ok(Token::ident(out, start))
}

/// Lex a boolean value.
fn lex_boolean(iter: &mut TokenIterator, character: char) -> Result<Token, Error> {
    let mut result = vec![character];
    let start = iter.location();

    while let Some(&next_character) = iter.char_iter.peek() {
        match next_character {
            'a'...'z' | 'A'...'Z' => {
                iter.next_location(IterateInternally::Yes);
                result.push(next_character);
            }
            // Stop on whitespace or parentheses.
            '(' | ')' | ' ' | '\n' | '\t' | '\r' => break,
            _ => return Err(Error::InvalidCharacter(next_character, start.0)),
        }
    }

    let out: String = result.iter().cloned().collect();
    let end = iter.location();

    if out == "#t" || out == "#true" {
        return Ok(Token::boolean(true, start, end));
    } else if out == "#f" || out == "#false" {
        return Ok(Token::boolean(false, start, end));
    } else {
        Err(Error::InvalidLiteral(out, start.0))
    }
}

/// Lex a string literal.
fn lex_string(iter: &mut TokenIterator, character: char) -> Result<Token, Error> {
    let mut result = Vec::new();
    let mut escape = false;
    let start = iter.location();

    while let Some(&next_character) = iter.char_iter.peek() {
        match next_character {
            '\\' if !escape => {
                escape = true;
            }
            '\\' if escape => {
                escape = false;
                iter.next_location(IterateInternally::Yes);
                result.push('\\');
            }
            't' if escape => {
                escape = false;
                iter.next_location(IterateInternally::Yes);
                result.push('\t');
            }
            'n' if escape => {
                escape = false;
                iter.next_location(IterateInternally::Yes);
                result.push('\n');
            }
            'r' if escape => {
                escape = false;
                iter.next_location(IterateInternally::Yes);
                result.push('\r');
            }
            x if character == x && escape => {
                iter.next_location(IterateInternally::Yes);
                result.push(x)
            }
            x if character == x && !escape => {
                iter.next_location(IterateInternally::Yes);
                break;
            }
            _ if escape => {
                let sequence = format!("\\{}", next_character);
                return Err(Error::InvalidEscapeSequence(sequence, iter.location().0));
            }
            _ => {
                escape = false;
                iter.next_location(IterateInternally::Yes);
                result.push(next_character);
            }
        }
    }

    let out: String = result.iter().cloned().collect();
    Ok(Token::string(out, start))
}
