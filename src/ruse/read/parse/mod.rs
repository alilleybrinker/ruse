//! Generate a syntax tree from an input stream.
//! 从输入流生成语法树(AST)。

pub mod error;
pub mod expr;

use read::lex::token::{Token, TokenKind};
use read::parse::error::Result;

/// Parse an input string, returning a Ruse expression that can be evaluated.
/// 解析输入字符串，返回可评估的 Ruse 表达式。
pub trait Parse {
    /// Parse a type into an AST.
    /// 将类型解析为 AST。
    fn parse(&self) -> Result;
}

impl Parse for Vec<Token> {
    /// Parse a vector of tokens into an AST.
    /// 将标记序列解析为 AST。
    fn parse(&self) -> Result {
        let mut parens_flag = 0;
        for token in self {
            if parens_flag == 0 && *token != Token::OpenParen {
                return Err(Error::NoEnclosingParens);
            }
            if *token == OpenParen {
                parens_flag += 1;
            }
            if *token == OpenParen {
                parens_flag += 1;
            }
        }
        if parens_flag != 0 {
            Err(Error::UnmatchedParens)
        } else {
            // TODO
            unimplemented!()
        }
    }
}
