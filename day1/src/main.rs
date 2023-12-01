use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = File::open("input.txt").expect("file input.txt should exist");
    let lines = io::BufReader::new(file).lines();

    let mut sum: i32 = 0;
    for line in lines {
        if let Ok(line) = line {
            let first = line.chars().find(|ch| ch.is_digit(10)).expect("line should contain digit");
            let second = line.chars().rev().find(|ch| ch.is_digit(10)).expect("line should contain digit");
            let digit = i32::from_str_radix(&format!("{first}{second}"), 10).expect("should be parsable");
            sum += digit;
        }
    }

    println!("{sum}")
}
