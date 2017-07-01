extern crate libruse;
extern crate clap;

use libruse::Engine;
use clap::{Arg, App};

fn main() {
    let matches = App::new("ruse")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Ruse Language Developers")
        .about("An embedded Scheme for Rust")
		.arg(Arg::with_name("SOURCE")
			.help("The Ruse program to execute")
			.required(true)
			.index(1))
		.get_matches();

	let source = matches.value_of("SOURCE").expect("No program provided.");

    let mut engine = Engine::new();
    let result = engine.run(source);
    println!("{}", result.unwrap());
}
