use parse::{LexResult, LexError, Token};
use std::cell::Cell;
use std::iter::Peekable;
use std::str::Chars;

pub struct Lexer {}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer {}
    }

    pub fn lex<'a>(&'a self, s: &'a str) -> LexResult {
        TokenIterator::new(s).collect::<LexResult>()
    }
}

pub struct TokenIterator<'a> {
    char_iter: Peekable<Chars<'a>>,
    location: Cell<usize>,
    first: Cell<bool>,
}

pub type TokenResult = Result<Token, LexError>;

impl<'a> TokenIterator<'a> {
    pub fn new<'b>(s: &'b str) -> TokenIterator<'b> {
        TokenIterator {
            char_iter: s.chars().peekable(),
            location: Cell::new(0),
            first: Cell::new(true),
        }
    }

    fn increment_location(&self) {
        let old_location = self.location.get();
        self.location.set(old_location + 1);
    }

    fn parse_open_paren(&self) -> TokenResult {
        Ok(Token::open_paren(self.location.get()))
    }

    fn parse_close_paren(&self) -> TokenResult {
        Ok(Token::close_paren(self.location.get()))
    }

    fn parse_number(&mut self, character: char) -> TokenResult {
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

    fn parse_identifier(&mut self, character: char) -> TokenResult {
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
                _ => break,
            }
        }

        let out: String = result.iter().cloned().collect();
        Ok(Token::ident(out, self.location.get()))
    }
}

impl<'a> Iterator for TokenIterator<'a> {
    type Item = TokenResult;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(character) = self.char_iter.next() {
            self.increment_location();

            if self.first.get() {
                self.location.set(0);
                self.first.set(false);
            }

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
