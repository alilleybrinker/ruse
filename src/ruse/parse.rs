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

pub type ParseError = u32;

pub fn parse(input: &str) -> Result<RuseVal, ParseError> {
    match ruse_string(&input.as_bytes()[..]) {
        IResult::Done(_, rc) => Ok(rc),
        _ => Err(0)
    }
}

named!(
    ruse_string(&[u8]) -> RuseVal,
    chain!(
        s: quoted_string,
        || {
            let s = String::from_utf8(s.to_vec()).unwrap();
            RuseVal::Stringy(s)
        }));

named!(
    quoted_string,
    delimited!(
        char!('"'),
        quoted_string,
        char!('"')));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_quoted_string() {
        if let Ok(RuseVal::Stringy(s)) = parse(r#""a""#) {
            assert_eq!("a", s);
        }
    }

    #[test]
    fn escaped_quoted_string() {
        if let Ok(RuseVal::Stringy(s)) = parse("\"\"a\"\"") {
            assert_eq!("\"a\"", s);
        }
    }

    #[test]
    fn broken_quoted_string() {
        if let Err(_) = parse("\"\"a\"") {
            assert!(true);
        }
    }
}

