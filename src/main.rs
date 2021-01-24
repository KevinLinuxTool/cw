extern crate chrono;

use std::{env, io};
use std::io::BufRead;
use chrono::Local;

fn main() {
    let args: Vec<String> = env::args().collect();
    let stdin = io::stdin();

    if args.len() != 2 {
        panic!("Number of arguments must be 1!");
    }

    let parse_str = args[1].clone();
    let mut stdin_str = String::new();
    for line in stdin.lock().lines() {
        stdin_str.push_str(&*line.expect(""));
    }

    println!("{}", parse(&parse_str, &stdin_str));
}

fn parse(parse_str: &str, stdin_str: &str) -> String {
    let date = Local::now();
    let mut result: Vec<String> = Vec::new();
    let split = parse_str.split("{{");
    for i in split {
        let ss = i.split("}}");
        for s in ss {
            result.push(
                if s == "{}" {
                    String::from(stdin_str)
                } else if s.starts_with("{") && s.ends_with("}") {
                    date.format(&s[1..s.len() - 1]).to_string()
                } else {
                    String::from(s)
                });
            result.push(String::from("}"));
        }
        result.pop();
        result.push(String::from("{"));
    }
    result.pop();
    let mut result_str = String::new();
    for rstr in result {
        result_str.push_str(&rstr);
    };
    result_str
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

    fn from_str(str: &str) -> Self {
        Queue {
            data: str.chars().collect::<Vec<_>>()
        }
    }
}
