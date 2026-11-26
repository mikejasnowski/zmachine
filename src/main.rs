
// @Todo: Review package scoping, modules later
use zmachine::ZMachine;
use std::env;

fn main() {
    println!("Z-Machine!");

    // Command-Line arguments
    let args: Vec<String> = env::args().collect();

    // @Todo: Review self vs associated methods
    let zm = ZMachine::new(args[1].clone());
    zm.load_story();
}

