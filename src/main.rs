use std::{env, io};
use std::io::BufRead;

fn main() {
    let args : Vec<String> = env::args().collect();
    let stdin = io::stdin();
    let stdout = io::stdout();

    for line in stdin.lock().lines() {
        let line = line.expect("NONE");
    }
}
