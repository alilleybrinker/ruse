extern crate ruse;
use ruse::engine::Engine;

fn main() {
    let mut engine = Engine::new();
    let result = engine.run("(+ 2 3)");
    println!("{}", result.unwrap());
}
