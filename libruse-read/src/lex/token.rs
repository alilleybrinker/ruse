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
    pub line: usize,
    pub column: usize,
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

