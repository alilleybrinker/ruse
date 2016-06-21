use parse;
use lex::Lexer;

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
    ///     [<open_paren>, <ident:+>, <int_literal: 2>, <int_literal: 3>, <close_paren>]
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
    pub fn parse<S: AsRef<str>>(&mut self, s: S) -> parse::Result {
        let lexer = Lexer::new();
        let tokens = try!(lexer.lex(s.as_ref()));

        println!("{:?} {:?}", s.as_ref(), tokens);

        // Tokens will be a Vec<Token>, which should then be parsed into a
        // SyntaxTree.
        unimplemented!()
    }
}
