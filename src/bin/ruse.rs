extern crate ruse;
use ruse::*;

fn main() {
    //let result = symbol("!");
    //println!("{:?}", result);

    let result = quoted_string(&"\"a\"".as_bytes());
    println!("{:?}", String::from_utf8_unchecked(result));
}


