extern crate chrono;

use std::{env, io};
use std::io::BufRead;
use chrono::Local;

fn main() {
    let args: Vec<String> = env::args().collect();
    let stdin = io::stdin();
    let stdout = io::stdout();

    println!("{}", parse("{}{{%Y-%m-%d][%H:%M:%S}}{%Y-%m-%d %H:%M:%S}", "2333"))
    //for line in stdin.lock().lines() {
    //    let line = line.expect("NONE");
    //}
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
