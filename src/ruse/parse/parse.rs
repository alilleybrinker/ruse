use parse::{ParseResult, TokenIterator, lex};

pub struct Parser {}

impl Parser {
    pub fn new() -> Parser {
        Parser {}
    }

    /// This function takes in a string representing a ruse expression, and
    /// parses it into an abstract syntax tree.
    ///
    /// The SyntaxTree is a tree of SyntaxNodes, each of which contains the
    /// original text and some data associated with the text.
    ///
    /// For example, the following program:
    ///
    ///     (+ 2 3)
    ///
    /// Becomes:
    ///
    ///         <fn: '+'>
    ///         /       \
    /// <atom: 2>       <atom: 3>
    ///
    /// For now, this is done by assuming that the first item after an open
    /// paren is a function call, and that everything after is an atom. This
    /// will obviously become better over time.
    pub fn parse<S: AsRef<str>>(&mut self, s: S) -> ParseResult {
        self.parse_tokens(lex(s.as_ref()))
    }

    fn parse_tokens<'a>(&mut self, s: TokenIterator<'a>) -> ParseResult {
        unimplemented!();
    }
}
