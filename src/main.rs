use pwgen::*;
use std::process;

fn main() {
    // Parse cli arguments
    let options = pwgen::Options::from_args();

    if let Err(e) = run(&options) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
