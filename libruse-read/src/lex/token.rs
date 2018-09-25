use lex::error::Error;
use std::cell::Cell;
use std::iter::Peekable;
use std::str::Chars;
use std::iter::Iterator;
use std::fmt;
use std::convert::Into;

#[derive(PartialEq, Clone)]
pub enum TokenKind {
    OpenDelim(Delim),
    CloseDelim(Delim),
    Symbol(String),
    Integer(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    // TODO: Implement line comments
    LineComment(String),
    // TODO: Implement block comments
    BlockComment(String),
    // TODO: Implement datum comments
    DatumComment(String),
}

impl fmt::Debug for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenKind::OpenDelim(d) => {
                match d {
                    Delim::Paren   => write!(f, "'('"),
                    Delim::Bracket => write!(f, "'['"),
                    Delim::Brace   => write!(f, "'{{'"),
                }
            }
            TokenKind::CloseDelim(d) => {
                match d {
                    Delim::Paren   => write!(f, "')'"),
                    Delim::Bracket => write!(f, "']'"),
                    Delim::Brace   => write!(f, "'}}'"),
                }
            }
            TokenKind::Symbol(s) => {
                write!(f, "'{}'", s.clone())
            }
            TokenKind::Integer(i) => {
                write!(f, "'{}'", *i)
            }
            TokenKind::Float(fl) => {
                write!(f, "'{}'", fl)
            }
            TokenKind::Str(s) => {
                write!(f, "\"{}\"", s.clone())
            }
            TokenKind::Bool(b) => {
                if *b {
                    write!(f, "#t")
                } else {
                    write!(f, "#f")
                }
            }
            TokenKind::LineComment(..) |
                TokenKind::BlockComment(..) |
                TokenKind::DatumComment(..) => write!(f, "")
        }
    }
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

#[derive(PartialEq, Clone, Copy)]
pub struct Location {
    line: usize,
    column: usize,
}

impl Location {
    pub fn new(line: usize, column: usize) -> Location {
        Location { line, column }
    }

    pub fn next_column(&mut self) {
        self.column += 1;
    }

    pub fn next_line(&mut self) {
        self.line += 1;
        self.column = 1;
    }
}

impl Default for Location {
    fn default() -> Location {
        Location {
            line: 1,
            column: 1,
        }
    }
}

impl fmt::Debug for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "line {}, column {}", self.line, self.column)
    }
}

#[derive(PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub start_location: Location,
    pub end_location: Location,
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<k:{:?}, s:{:?}, e:{:?}>", self.kind, self.start_location, self.end_location)
    }
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

    stringy_token!(symbol, Symbol);
    stringy_token!(string, Str);

    literal_token!(integer, Integer, i64);
    literal_token!(float, Float, f64);
    literal_token!(boolean, Bool, bool);
}

pub struct TokenIterator<'a> {
    char_iter: Peekable<Chars<'a>>,
    location: Cell<Location>,
    pub string_context: bool,
}

#[derive(Debug, PartialEq, Eq)]
enum EndOfLine { Yes, No }

impl Into<bool> for EndOfLine {
    fn into(self) -> bool {
        match self {
            EndOfLine::Yes => true,
            EndOfLine::No => false,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Move { Yes, No }

impl Into<bool> for Move {
    fn into(self) -> bool {
        match self {
            Move::Yes => true,
            Move::No => false,
        }
    }
}

impl<'a> TokenIterator<'a> {
    pub fn new(s: &str) -> TokenIterator {
        TokenIterator {
            char_iter: s.chars().peekable(),
            location: Cell::new(Location::default()),
            string_context: false,
        }
    }

    fn get_location(&self) -> Location {
        self.location.get()
    }

    fn set_location(&mut self, location: Location) {
        self.location.set(location);
    }

    fn step(&mut self, end_of_line: EndOfLine, m: Move) {
        let mut location = self.get_location();

        if end_of_line.into() && !self.string_context {
            location.next_line();
        } else {
            location.next_column();
        }

        self.set_location(location);

        if m.into() {
            self.char_iter.next();
        }
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

            // Decide what to attempt to lex based on the first character. Note that the
            // allowable characters for symbols are fewer here then they are in the
            // lex_symbol function.
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
                    '>' | '?' | '^' | '_' | '~' | '+' | '-' => return Some(lex_symbol(self, character)),
                '"' => return Some(lex_string(self, character)),
                // Skip whitespace.
                ' ' | '\t' | '\r' => self.step(EndOfLine::No, Move::No),
                // Count newlines
                '\n' => self.step(EndOfLine::Yes, Move::No),
                _ => return Some(Err(Error::InvalidCharacter(character, self.get_location()))),
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
        fn $name(iter: &mut TokenIterator) -> Result<Token, Error> {
            let start_location = iter.get_location();
            let end_location = Location {
                column: start_location.column + $span,
                line: start_location.line,
            };

            iter.step(EndOfLine::No, Move::No);
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
    let start = iter.get_location();

    while let Some(&next_character) = iter.char_iter.peek() {
        match next_character {
            '0'...'9' | '.' => {
                iter.step(EndOfLine::No, Move::Yes);
                result.push(next_character);
            }
            _ => break,
        }
    }

    iter.step(EndOfLine::No, Move::No);

    let out: String = result.iter().cloned().collect();
    let end = iter.get_location();

    if let Ok(val) = out.parse::<i64>() {
        Ok(Token::integer(val, start, end))
    } else if let Ok(val) = out.parse::<f64>() {
        Ok(Token::float(val, start, end))
    } else {
        Err(Error::MalformedNumber(out, start))
    }
}

fn lex_symbol(iter: &mut TokenIterator, character: char) -> Result<Token, Error> {
    let mut result = vec![character];
    let start = iter.get_location();

    while let Some(&next_character) = iter.char_iter.peek() {
        match next_character {
            'a'...'z' | 'A'...'Z' | '!' | '$' | '%' | '&' | '*' | '/' | ':' | '<' | '=' | '>' |
                '?' | '^' | '_' | '~' | '0'...'9' | '+' | '-' | '.' | '@' => {
                    iter.step(EndOfLine::No, Move::Yes);
                    result.push(next_character);
                }
            // Stop on whitespace or parentheses.
            '(' | ')' | ' ' | '\n' | '\t' | '\r' => break,
            _ => return Err(Error::InvalidCharacter(next_character, start)),
        }
    }

    iter.step(EndOfLine::No, Move::No);

    let out: String = result.iter().cloned().collect();

    Ok(Token::symbol(out, start))
}

fn lex_boolean(iter: &mut TokenIterator, character: char) -> Result<Token, Error> {
    let mut result = vec![character];
    let start = iter.get_location();

    while let Some(&next_character) = iter.char_iter.peek() {
        match next_character {
            'a'...'z' | 'A'...'Z' => {
                iter.step(EndOfLine::No, Move::Yes);
                result.push(next_character);
            }
            // Stop on whitespace or parentheses.
            '(' | ')' | ' ' | '\n' | '\t' | '\r' => break,
            _ => return Err(Error::InvalidCharacter(next_character, start)),
        }
    }

    iter.step(EndOfLine::No, Move::No);

    let out: String = result.iter().cloned().collect();
    let end = iter.get_location();

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
    let start = iter.get_location();

    iter.string_context = true;

    while let Some(&next_character) = iter.char_iter.peek() {
        match next_character {
            '\\' if !escape => {
                escape = true;
            }
            '\\' if escape => {
                escape = false;
                iter.step(EndOfLine::No, Move::Yes);
                result.push('\\');
            }
            't' if escape => {
                escape = false;
                iter.step(EndOfLine::No, Move::Yes);
                result.push('\t');
            }
            'n' if escape => {
                escape = false;
                iter.step(EndOfLine::No, Move::Yes);
                result.push('\n');
            }
            'r' if escape => {
                escape = false;
                iter.step(EndOfLine::No, Move::Yes);
                result.push('\r');
            }
            x if character == x && escape => {
                escape = false;
                iter.step(EndOfLine::No, Move::Yes);
                result.push(x)
            }
            x if character == x && !escape => {
                iter.step(EndOfLine::No, Move::Yes);
                break;
            }
            _ if escape => {
                let sequence = format!("\\{}", next_character);
                return Err(Error::InvalidEscapeSequence(sequence, iter.get_location()));
            }
            _ => {
                escape = false;
                iter.step(EndOfLine::No, Move::Yes);
                result.push(next_character);
            }
        }
    }

    iter.string_context = false;
    iter.step(EndOfLine::No, Move::No);

    let out: String = result.iter().cloned().collect();
    Ok(Token::string(out, start))
}

