//! A single lexical item.

/// The different types of Token. Some token kinds require additional data,
/// and will carry it here.
#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    /// A single open delimiter.
    OpenDelim(Delim),
    /// A single close delimiter.
    CloseDelim(Delim),
    /// An identifier.
    Ident(String),
    /// An integer literal.
    Integer(i64),
    /// A floating point literal.
    Float(f64),
    /// A string
    Str(String),
    /// A boolean
    Bool(bool),
}

impl TokenKind {
    /// Checks whether the token kind is an open delim.
    pub fn is_open_delim(&self) -> bool {
        match *self {
            TokenKind::OpenDelim(..) => true,
            _ => false,
        }
    }
}

/// Indicates type of delimiter.
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Delim {
    /// Parens: ( and )
    Paren,
    /// Brackets: [ and ]
    Bracket,
    /// Braces: { and }
    Brace,
}

/// The location of a token in an input stream.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Location(pub usize);

impl Location {
    /// Get the length of the span from the current location to the end
    /// location.
    pub fn to(&self, end_location: Location) -> Span {
        Span(end_location.0 - self.0 + 1)
    }
}

/// The span of a token in an input stream.
#[derive(Debug, PartialEq)]
pub struct Span(pub usize);

/// Tokens, generated by the lexer. Every token has a location in the input
/// stream, a span indicating how large they are, and a type that potentially
/// has some associated data.
#[derive(Debug, PartialEq)]
pub struct Token {
    /// The kind of token, potentially with some additional data.
    pub kind: TokenKind,
    /// The location of the token in the input stream.
    pub location: Location,
    /// The width of the token in the input stream.
    pub span: Span,
}

macro_rules! delim_token {
    ( $fn_name:ident, $open_or_closed:ident, $variant_name:ident ) => {
        /// Create a delimiter token.
        pub fn $fn_name(location: Location) -> Token {
            Token {
                kind: TokenKind::$open_or_closed(Delim::$variant_name),
                location: location,
                span: Span(1),
            }
        }
    };
}

macro_rules! stringy_token {
    ( $fn_name:ident, $variant_name:ident ) => {
        /// Create a stringy token.
        pub fn $fn_name<S: Into<String>>(s: S, location: Location) -> Token {
            let s: String = s.into();
            let len = s.len();

            Token {
                kind: TokenKind::$variant_name(s),
                location: location,
                span: Span(len),
            }
        }
    };
}

macro_rules! literal_token {
    ( $fn_name:ident, $variant_name:ident, $value_type:ty ) => {
        /// Create a literal token.
        pub fn $fn_name(value: $value_type, start_location: Location, end_location: Location) -> Token {
            Token {
                kind: TokenKind::$variant_name(value),
                location: start_location,
                span: start_location.to(end_location),
            }
        }
    };
}

impl Token {
    delim_token!(open_paren, OpenDelim, Paren);
    delim_token!(close_paren, CloseDelim, Paren);
    delim_token!(open_bracket, OpenDelim, Bracket);
    delim_token!(close_bracket, CloseDelim, Bracket);
    delim_token!(open_brace, OpenDelim, Brace);
    delim_token!(close_brace, CloseDelim, Brace);

    stringy_token!(ident, Ident);
    stringy_token!(string, Str);

    literal_token!(integer, Integer, i64);
    literal_token!(float, Float, f64);
    literal_token!(boolean, Bool, bool);
}
