use lex::{LexResult, LexError, Token};
use std::cell::Cell;
use std::iter::Peekable;
use std::str::Chars;

/// Lexes an input string to get a vector of tokens from it.
pub struct Lexer {}

impl Lexer {
    /// Get a new Lexer.
    pub fn new() -> Lexer {
        Lexer {}
    }

    /// Get a vector of tokens from the given string, or a LexError if there's
    /// something wrong with the input stream.
    pub fn lex(&self, s: &str) -> LexResult {
        TokenIterator::new(s).collect::<LexResult>()
    }
}

/// Iterator over tokens in a string.
pub struct TokenIterator<'a> {
    char_iter: Peekable<Chars<'a>>,
    location: Cell<usize>,
}

impl<'a> TokenIterator<'a> {
    /// Create a new TokenIterator to iterate over the given string.
    pub fn new<'b>(s: &'b str) -> TokenIterator<'b> {
        TokenIterator {
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
    fn parse_open_paren(&self) -> Result<Token, LexError> {
        Ok(Token::open_paren(self.location.get()))
    }

    /// Parse a closed parenthese.
    fn parse_close_paren(&self) -> Result<Token, LexError> {
        Ok(Token::close_paren(self.location.get()))
    }

    /// Parse a number, either floating point or integer.
    fn parse_number(&mut self, character: char) -> Result<Token, LexError> {
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

        Err(LexError::MalformedNumber(out))
    }

    /// Parse an identifier.
    fn parse_identifier(&mut self, character: char) -> Result<Token, LexError> {
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
                _ => return Err(LexError::InvalidCharacter(next_character, self.location.get())),
            }
        }

        let out: String = result.iter().cloned().collect();
        Ok(Token::ident(out, self.location.get()))
    }
}

impl<'a> Iterator for TokenIterator<'a> {
    type Item = Result<Token, LexError>;

    /// Returns one of three things:
    ///
    /// 1. `Option::None`
    /// 2. `Option::Some(Result::Err(LexError))`
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
