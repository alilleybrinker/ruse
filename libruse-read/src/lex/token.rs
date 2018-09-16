use lex::error::Error;
use std::cell::Cell;
use std::iter::Peekable;
use std::str::Chars;
use std::iter::Iterator;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    OpenDelim(Delim),
    CloseDelim(Delim),
    Ident(String),
    Integer(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    // TODO: Implement block comments
    BlockComment(String),
}

impl TokenKind {
    pub fn is_open_delim(&self) -> bool {
        match *self {
            TokenKind::OpenDelim(..) => true,
            _ => false,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Delim {
    Paren,
    Bracket,
    Brace,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Location {
    column: usize,
    line: usize,
}

impl Default for Location {
    fn default() -> Location {
        Location {
            line: 0,
            column: 0,
        }
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "line {}, column {}", self.line, self.column)
    }
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub start_location: Location,
    pub end_location: Location,
}

macro_rules! delim_token {
    ( $fn_name:ident, $kind:expr ) => {
        pub fn $fn_name(start_location: Location, end_location: Location) -> Token {
            Token {
                kind: $kind,
                start_location: start_location,
                end_location: end_location,
            }
        }
    };
}

macro_rules! stringy_token {
    ( $fn_name:ident, $variant_name:ident ) => {
        pub fn $fn_name<S: Into<String>>(s: S, start_location: Location) -> Token {
            let s: String = s.into();

            let end_location = Location {
                column: start_location.column + s.len(),
                line: start_location.line,
            };

            Token {
                kind: TokenKind::$variant_name(s),
                start_location: start_location,
                end_location: end_location,
            }
        }
    };
}

macro_rules! literal_token {
    ( $fn_name:ident, $variant_name:ident, $value_type:ty ) => {
        pub fn $fn_name(value: $value_type,
                        start_location: Location,
                        end_location: Location) -> Token {
            Token {
                kind: TokenKind::$variant_name(value),
                start_location: start_location,
                end_location: end_location,
            }
        }
    };
}

impl Token {
    delim_token!(open_paren, TokenKind::OpenDelim(Delim::Paren));
    delim_token!(close_paren, TokenKind::CloseDelim(Delim::Paren));
    delim_token!(open_bracket, TokenKind::OpenDelim(Delim::Bracket));
    delim_token!(close_bracket, TokenKind::CloseDelim(Delim::Bracket));
    delim_token!(open_brace, TokenKind::OpenDelim(Delim::Brace));
    delim_token!(close_brace, TokenKind::CloseDelim(Delim::Brace));

    stringy_token!(ident, Ident);
    stringy_token!(string, Str);

    literal_token!(integer, Integer, i64);
    literal_token!(float, Float, f64);
    literal_token!(boolean, Bool, bool);
}

pub struct TokenIterator<'a> {
    char_iter: Peekable<Chars<'a>>,
    location: Cell<Location>,
}

enum IterateInternally {
    Yes,
    No,
}

impl<'a> TokenIterator<'a> {
    pub fn new(s: &str) -> TokenIterator {
        TokenIterator {
            char_iter: s.chars().peekable(),
            location: Cell::new(Location::default()),
        }
    }

    fn next_location(&mut self, go_to_next: IterateInternally) {
        if self.char_iter.peek().filter(|v| **v == '\n').is_some() {
            let new_location = Location {
                column: 0,
                line: self.location().line + 1,
            };

            self.location.set(new_location);
        }

        if let IterateInternally::Yes = go_to_next {
            self.char_iter.next();
        }
    }

    fn location(&self) -> Location {
        self.location.get()
    }
}

impl<'a> Iterator for TokenIterator<'a> {
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
            // lex_ident function.
            match character {
                '(' => return Some(lex_open_paren(self)),
                ')' => return Some(lex_closed_paren(self)),
                '[' => return Some(lex_open_bracket(self)),
                ']' => return Some(lex_closed_bracket(self)),
                '{' => return Some(lex_open_brace(self)),
                '}' => return Some(lex_closed_brace(self)),
                '#' => return Some(lex_boolean(self, character)),
                '0'...'9' => return Some(lex_number(self, character)),
                'a'...'z' | 'A'...'Z' | '!' | '$' | '%' | '&' | '*' | '/' | ':' | '<' | '=' |
                    '>' | '?' | '^' | '_' | '~' | '+' | '-' => return Some(lex_ident(self, character)),
                '"' => return Some(lex_string(self, character)),
                // Skip whitespace.
                ' ' | '\n' | '\t' | '\r' => (),
                _ => return Some(Err(Error::InvalidCharacter(character, self.location()))),
            }
        }

        None
    }
}

pub trait StrTokenIterator: AsRef<str> {
    fn tokens(&self) -> TokenIterator {
        TokenIterator::new(self.as_ref())
    }
}

// Implement StrIterExt for all types that can be ref'ed into string slices.
impl<T: AsRef<str>> StrTokenIterator for T {}

macro_rules! lex_delim {
    ($name:ident, $e:ident, $span:expr) => {
        fn $name(iter: &TokenIterator) -> Result<Token, Error> {
            let start_location = iter.location();
            let end_location = Location {
                column: start_location.column + $span,
                line: start_location.line,
            };

            Ok(Token::$e(start_location, end_location))
        }
    }
}

lex_delim!(lex_open_paren, open_paren, 1);
lex_delim!(lex_closed_paren, close_paren, 1);
lex_delim!(lex_open_bracket, open_bracket, 1);
lex_delim!(lex_closed_bracket, close_bracket, 1);
lex_delim!(lex_open_brace, open_brace, 1);
lex_delim!(lex_closed_brace, close_brace, 1);

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
        Err(Error::MalformedNumber(out, start))
    }
}

fn lex_ident(iter: &mut TokenIterator, character: char) -> Result<Token, Error> {
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
            _ => return Err(Error::InvalidCharacter(next_character, start)),
        }
    }

    let out: String = result.iter().cloned().collect();

    Ok(Token::ident(out, start))
}

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
            _ => return Err(Error::InvalidCharacter(next_character, start)),
        }
    }

    let out: String = result.iter().cloned().collect();
    let end = iter.location();

    if out == "#t" || out == "#true" {
        Ok(Token::boolean(true, start, end))
    } else if out == "#f" || out == "#false" {
        Ok(Token::boolean(false, start, end))
    } else {
        Err(Error::InvalidLiteral(out, start))
    }
}

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
                escape = false;
                iter.next_location(IterateInternally::Yes);
                result.push(x)
            }
            x if character == x && !escape => {
                iter.next_location(IterateInternally::Yes);
                break;
            }
            _ if escape => {
                let sequence = format!("\\{}", next_character);
                return Err(Error::InvalidEscapeSequence(sequence, iter.location()));
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
