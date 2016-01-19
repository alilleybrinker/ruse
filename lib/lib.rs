extern crate combine;
use combine::char::{char};
use combine::primitives::{Parser, Stream, ParseError};
use combine::combinator::{try, choice, between, many, any, ParserExt};


#[derive(Debug, PartialEq)]
pub enum RuseVal {
    Atom(String),
    List(Vec<Box<RuseVal>>),
    DottedList(Vec<Box<RuseVal>>, Box<RuseVal>),
    Number(i64),
    Stringy(String),
    Bool(bool),
}


/// Parser for Ruse's acceptable symbols:
///
///    "!#$%&|*+-/:<=>?@^_~"
pub fn symbol<I>(input: I)
             -> Result<(char, I), ParseError<I>>
             where I: Stream<Item=char> {
    let mut parser = choice([
        try(char('!')),
        try(char('#')),
        try(char('$')),
        try(char('%')),
        try(char('&')),
        try(char('|')),
        try(char('*')),
        try(char('+')),
        try(char('-')),
        try(char('/')),
        try(char(':')),
        try(char('<')),
        try(char('=')),
        try(char('>')),
        try(char('?')),
        try(char('@')),
        try(char('^')),
        try(char('_')),
        try(char('~')),
    ]);
    parser.parse(input)
}

/// Parser for quoted strings.
pub fn quoted_string<I>(input: I)
             -> Result<(RuseVal, I), ParseError<I>>
             where I: Stream<Item=char> {
    let mut parser = between(char('"'), char('"'), many(any()))
                     .map(RuseVal::Stringy);
    parser.parse(input)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_symbols() {
        assert_eq!(symbol("!"), Ok(('!', "")));
        assert_eq!(symbol("#"), Ok(('#', "")));
        assert_eq!(symbol("$"), Ok(('$', "")));
        assert_eq!(symbol("%"), Ok(('%', "")));
        assert_eq!(symbol("&"), Ok(('&', "")));
        assert_eq!(symbol("|"), Ok(('|', "")));
        assert_eq!(symbol("*"), Ok(('*', "")));
        assert_eq!(symbol("+"), Ok(('+', "")));
        assert_eq!(symbol("-"), Ok(('-', "")));
        assert_eq!(symbol("/"), Ok(('/', "")));
        assert_eq!(symbol(":"), Ok((':', "")));
        assert_eq!(symbol("<"), Ok(('<', "")));
        assert_eq!(symbol("="), Ok(('=', "")));
        assert_eq!(symbol(">"), Ok(('>', "")));
        assert_eq!(symbol("?"), Ok(('?', "")));
        assert_eq!(symbol("@"), Ok(('@', "")));
        assert_eq!(symbol("^"), Ok(('^', "")));
        assert_eq!(symbol("_"), Ok(('_', "")));
        assert_eq!(symbol("~"), Ok(('~', "")));
    }

    #[test]
    fn parse_quoted_strings() {
        assert_eq!(quoted_string(r#""#),
                   Ok((RuseVal::Stringy("".to_owned()), "")));
        assert_eq!(quoted_string(r#"blah"#),
                   Ok((RuseVal::Stringy("blah".to_owned()), "")));
    }
}


