use std::{env, io};
use std::io::BufRead;

fn main() {
    let args: Vec<String> = env::args().collect();
    let stdin = io::stdin();
    let stdout = io::stdout();

    for line in stdin.lock().lines() {
        let line = line.expect("NONE");
    }
}

fn parse<'a>(parse_str: &'a str, stdin_str: &'a str) -> &'a str {
    // {yy}, {yyyy} -> year
    // {m},  {mm} -> month
    // {d},  {dd} -> day
    let mut q: Queue = Queue::from(parse_str);
    while q.len() > 0 {
        println!("{}", q.pop())
    }
    parse_str
}

#[derive(Debug)]
struct Queue {
    data: Vec<char>,
}

impl Queue {
    fn len(&self) -> usize {
        self.data.len()
    }

    fn new() -> Self {
        Queue { data: Vec::new() }
    }

    fn from(from: &str) -> Self {
        Queue {
            data: from.chars().collect::<Vec<_>>()
        }
    }

    fn push(&mut self, item: char) {
        self.data.push(item);
    }

    fn pop(&mut self) -> char {
        self.data.remove(0)
    }
}
