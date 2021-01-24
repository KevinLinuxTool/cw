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
