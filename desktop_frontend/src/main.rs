// crates
use std::env;

fn main() {
    // get arguments from command line
    let arguments: Vec<_> = env::args().collect();

    // must only have the game path, no other arguments
    if arguments.len() != 2 {
        println!("Usage: cargo run path_to_game");
        return;
    }
}
