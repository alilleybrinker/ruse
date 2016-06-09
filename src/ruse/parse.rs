use stack::Stack;
use std::iter::Peekable;

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
    pub fn parse<S: AsRef<str>>(&mut self, s: S) -> SyntaxTree {
        self.parse_peekable(s.as_ref().chars().peekable())
    }

    fn parse_peekable<I: Iterator>(&mut self, s: Peekable<I>) -> SyntaxTree {
        unimplemented!();
    }
}

pub enum SyntaxNode {
    Function(String),
    Atom(i64),
}

pub struct SyntaxTree {
    tree: Stack<SyntaxNode>,
}

impl SyntaxTree {
    pub fn new() -> SyntaxTree {
        SyntaxTree { tree: Stack::new() }
    }
}

impl Into<String> for SyntaxTree {
    fn into(self) -> String {
        "Syntax Tree".to_string()
    }
}
