extern crate ruse;
use ruse::*;

fn main() {
    let result = symbol("!");
    println!("{:?}", result);

    let result = quoted_string("\"a\"");
    println!("{:?}", result);
}


