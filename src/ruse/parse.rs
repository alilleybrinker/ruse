
pub struct Parser {}

impl Parser {
    pub fn new() -> Parser {
        Parser {}
    }

    pub fn parse<S: AsRef<str>>(&mut self, s: S) -> SyntaxTree {
        SyntaxTree {}
    }
}

pub struct SyntaxTree {}

impl Into<String> for SyntaxTree {
    fn into(self) -> String {
        "Syntax Tree".to_string()
    }
}
