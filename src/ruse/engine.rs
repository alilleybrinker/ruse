use parse::{Parser, SyntaxTree};

pub struct Engine {}

impl Engine {
    pub fn new() -> Engine {
        Engine {}
    }

    pub fn eval<S: AsRef<str>>(&mut self, s: S) -> String {
        let mut parser: Parser = Parser::new();
        let result: String = parser.parse(s.as_ref()).into();
        result
    }
}
