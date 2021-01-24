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
    let mut result: Vec<String> = Vec::new();
    let split = parse_str.split("{{");
    for i in split {
        let ss = i.split("}}");
        for s in ss {
            result.push(parse_var(s, stdin_str));
            result.push(String::from("}"));
        }
        result.pop();
        result.push(String::from("{"));
    }
    result.pop();
    let mut result_str = String::new();
    for r_str in result {
        result_str.push_str(&r_str);
    };
    result_str
}

fn parse_var(str: &str, stdin: &str) -> String {
    let date = Local::now();
    let mut q: Queue<char> = Queue::from_str(str);
    let mut r = String::new();
    let mut is_under_scope = false;
    let mut scope: Queue<char> = Queue::new();
    while q.len() != 0 {
        let c = q.pop();
        match c {
            '{' => {
                is_under_scope = true;
                scope.clear();
            }
            '}' => {
                is_under_scope = false;
                let sc = scope.to_string();
                match &sc[..] {
                    "" => r.push_str(stdin),
                    _ => r.push_str(&date.format(&sc[..]).to_string())
                };
            }
            _ => {
                if is_under_scope {
                    scope.push(c);
                } else {
                    r.push(c);
                }
            }
        };
    };
    r.to_string()
}

#[derive(Debug)]
struct Queue<T> {
    data: Vec<T>,
}

#[allow(dead_code)]
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

    fn clear(&mut self) {
        self.data.clear();
    }
}

#[allow(dead_code)]
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
