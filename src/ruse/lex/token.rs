#[derive(Debug)]
pub enum TokenKind {
    OpenParen,
    CloseParen,
    Ident(String),
    IntegerLiteral(i64),
    FloatLiteral(f64),
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub location: usize,
    pub span: usize,
}

impl Token {
    pub fn open_paren(location: usize) -> Token {
        Token {
            kind: TokenKind::OpenParen,
            location: location,
            span: 1,
        }
    }

    pub fn close_paren(location: usize) -> Token {
        Token {
            kind: TokenKind::CloseParen,
            location: location,
            span: 1,
        }
    }

    pub fn ident(name: String, location: usize) -> Token {
        let len = name.len();
        Token {
            kind: TokenKind::Ident(name),
            location: location,
            span: len,
        }
    }

    pub fn integer(value: i64, span: usize, location: usize) -> Token {
        Token {
            kind: TokenKind::IntegerLiteral(value),
            location: location,
            span: span,
        }
    }

    pub fn float(value: f64, span: usize, location: usize) -> Token {
        Token {
            kind: TokenKind::FloatLiteral(value),
            location: location,
            span: span,
        }
    }
}
