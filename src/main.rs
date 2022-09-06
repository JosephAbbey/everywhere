use std::env;

use everywhere::hi;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{}", hi(args[1].to_string()));
}
