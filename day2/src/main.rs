use std::fs::File;
use std::io::{self, BufRead};

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

fn main() {
    let file = File::open("input.txt").expect("file input.txt should exist");
    let lines = io::BufReader::new(file).lines();

    let mut sum = 0;

    'next_line: for (i, line) in lines.enumerate() {
        let line = line.expect("line should be readable");
        let mut iter = line.split_whitespace().skip(2);
        
        while let Some(amount) = iter.next() {
            let amount = amount.parse::<u32>().expect("should be able to parse a number");
            
            let color = iter.next().expect("color should follow a number");
            match color.chars().nth(0) {
                Some('r') => if amount > MAX_RED { continue 'next_line; },
                Some('b') => if amount > MAX_BLUE { continue 'next_line; },
                Some('g') => if amount > MAX_GREEN { continue 'next_line; },
                _ => panic!("color should follow a number")
            }
        }

        sum += i + 1;
    }

    println!("{sum}");
}
