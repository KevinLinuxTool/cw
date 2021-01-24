use std::{env, io};
use std::io::BufRead;
use std::iter::FromIterator;

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
    let mut q: Queue<char> = Queue::from(parse_str.chars().collect::<Vec<_>>());
    while q.len() > 0 {
        println!("{}", q.pop())
    }
    parse_str
}

#[derive(Debug)]
struct Queue<T> {
    data: Vec<T>,
}

impl<T> Queue<T> {
    fn len(&self) -> usize {
        self.data.len()
    }

    fn new() -> Self {
        Queue { data: Vec::new() }
    }

    fn from(from: Vec<T>) -> Self {
        Queue {
            data: from
        }
    }

    fn push(&mut self, item: T) {
        self.data.push(item);
    }

    fn pop(&mut self) -> T {
        self.data.remove(0)
    }
}

impl Queue<char> {
    fn to_string(&self) -> String {
        self.data.iter().collect::<String>()
    }
}
