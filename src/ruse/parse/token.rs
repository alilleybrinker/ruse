pub enum TokenKind<'a> {
    Paren,
    Ident(&'a str),
}

pub struct Token<'a> {
    pub kind: TokenKind<'a>,
    pub location: usize,
    pub span: usize,
}

impl<'a> Token<'a> {
    pub fn paren(location: usize) -> Token<'a> {
        Token {
            kind: TokenKind::Paren,
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
}
