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
    pub quoted_string,
    delimited!(
        char!('"'),
        take_until_either!("\""),
        char!('"')
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use nom::IResult;

    #[test]
    fn test_quoted_string() {
        if let IResult::Done(_, result) = quoted_string(b"\"a\"") {
            let result = String::from_utf8(result.to_vec()).unwrap();
            assert_eq!("a", result);
        }
    }
}

