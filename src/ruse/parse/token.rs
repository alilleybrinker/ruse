pub enum TokenKind<'a> {
    OpenParen,
    CloseParen,
    Ident(&'a str),
    IntegerLiteral(i64),
    FloatLiteral(f64),
}

pub struct Token<'a> {
    pub kind: TokenKind<'a>,
    pub location: usize,
    pub span: usize,
}

impl<'a> Token<'a> {
    pub fn open_paren(location: usize) -> Token<'a> {
        Token {
            kind: TokenKind::OpenParen,
            location: location,
            span: 1,
        }
    }

    pub fn close_paren(location: usize) -> Token<'a> {
        Token {
            kind: TokenKind::CloseParen,
            location: location,
            span: 1,
        }
    }

    pub fn ident(name: &'a str, location: usize) -> Token<'a> {
        Token {
            kind: TokenKind::Ident(name),
            location: location,
            span: name.len(),
        }
    }

    pub fn integer_literal(value: i64, span: usize, location: usize) -> Token<'a> {
        Token {
            kind: TokenKind::IntegerLiteral(value),
            location: location,
            span: span,
        }
    }

    pub fn float_literal(value: f64, span: usize, location: usize) -> Token<'a> {
        Token {
            kind: TokenKind::FloatLiteral(value),
            location: location,
            span: span,
        }
    }
}
