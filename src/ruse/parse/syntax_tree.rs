use stack::Stack;

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
