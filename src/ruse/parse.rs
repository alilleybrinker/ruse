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

    // TODO: Add tests here.
}

