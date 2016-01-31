extern crate ruse;
use ruse::*;

fn main() {
    if let IResult::Done(_, result) = quoted_string(b"\"a\"") {
        println!("{}", String::from_utf8(result.to_vec()).unwrap());
    }
}


