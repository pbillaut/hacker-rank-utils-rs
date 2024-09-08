#![deny(warnings)]
#![warn(rust_2018_idioms)]
#![allow(unused)]

use hacker_rank_utils_rs::InputParser;
use std::error::Error;
use std::io;
use std::str::FromStr;

#[derive(Debug)]
enum Operation {
    Append(String),
    Delete(u32),
    Print(u32),
    Undo,
}

impl FromStr for Operation {
    type Err = Box<dyn Error>;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let fields = line.split(' ').collect::<Vec<&str>>();
        match fields[0] {
            "1" => Ok(Self::Append(fields[1].to_string())),
            "2" => Ok(Self::Delete(fields[1].parse()?)),
            "3" => Ok(Self::Print(fields[1].parse()?)),
            "4" => Ok(Self::Undo),
            _ => Err(format!("Unknown operation: {:?}", fields).into()),
        }
    }
}

fn solution(operations: Vec<Operation>) {
    // Enter your code here
}

fn main() {
    let stdin = io::stdin();
    let mut reader = InputParser::new(stdin.lock());
    let operations = reader.next_vector().expect("Unable to parse input");
    solution(operations);
}
