#[macro_use]
extern crate nom;
use nom::IResult;


#[derive(Debug, PartialEq)]
pub enum RuseVal {
    Atom(String),
    List(Vec<Box<RuseVal>>),
    DottedList(Vec<Box<RuseVal>>, Box<RuseVal>),
    Number(i64),
    Stringy(String),
    Bool(bool),
}


named!(
    pub quoted_string<&[u8], String>,
    chain!(
        delimited!(
            char!('"'),
            many0!(is_not!("\"")),
            char!('"')
        ) ~
        map_res!(String::from_utf8)
    )
);

/*
/// Parser for quoted strings.
named!(
   pub quoted_string<&[u8], RuseVal>,
   chain!(
       delimited!(char!('"'), many0!(is_not!("\"")), char!('"')) ~
       map!(string_from_iresult) ,
       || { RuseVal::Stringy }
   )
);
*/


#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Add tests here.
}


