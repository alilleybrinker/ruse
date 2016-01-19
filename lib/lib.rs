extern crate combine;
use combine::char::char;
use combine::primitives::{Parser, Stream, ParseError};
use combine::combinator::{try, choice};


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


