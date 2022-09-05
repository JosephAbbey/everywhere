use std::env;

use everywhere::hello;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{}", hello(&args[1]));
}
