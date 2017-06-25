//! Generate a syntax tree from an input stream.
//! 从输入流生成语法树(AST)。

pub mod error;
pub mod expr;

use read::lex::token::Token;
use read::parse::error::Result;

/// Parse an input string, returning a Ruse expression that can be evaluated.
/// 解析输入字符串，返回可评估的 Ruse 表达式。
pub trait Parse {
    /// Parse a type into an AST.
    fn parse(&self) -> Result;
}

impl Parse for Vec<Token> {
    /// Parse a vector of tokens into an AST.
    /// 将标记序列解析为 AST。
    fn parse(&self) -> Result {
        unimplemented!()
    }
}
