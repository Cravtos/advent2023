use std::fs::File;
use std::io::{self, BufRead};

fn find_digit(mut line: &str, rev: bool) -> Option<u32> {
    let mut digits = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"].map(str::to_owned);
    if rev {
        digits = digits.map(|digit| digit.chars().rev().collect());
    }

    while !line.is_empty() {
        let ch = line.chars().next().expect("line should end here");
        let digit = ch.to_digit(10);
        if digit.is_some() {
            return digit;
        }

        for i in 0..digits.len() {
            if line.starts_with(&digits[i]) {
                return Some((i+1) as u32);
            }
        }

        line = &line[1..];
    }

    None
}

fn main() {
    let file = File::open("input.txt").expect("file input.txt should exist");
    let lines = io::BufReader::new(file).lines();

    let mut sum: u32 = 0;
    for line in lines.filter(|l| l.is_ok()) {
        let line = line.unwrap();

        let first: u32 = find_digit(&line, false).expect("line should contain digit");
        let last: u32 = find_digit(&line.chars().rev().collect::<String>(), true).expect("line should contain digit");
         
        sum += first * 10 + last;
    }

    println!("{sum}")
}
