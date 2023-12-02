use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let file = File::open("input.txt").expect("file input.txt should exist");
    let lines = io::BufReader::new(file).lines();

    let mut sum = 0;

    for line in lines {
        let line = line.expect("line should be readable");
        let mut iter = line.split_whitespace().skip(2);
        
        let mut mins = vec![0u32; 3];
        while let Some(amount) = iter.next() {
            let amount = amount.parse::<u32>().expect("should be able to parse a number");
            
            let color = iter.next().expect("color should follow a number");
            match color.chars().nth(0) {
                Some('r') => if amount > mins[0] { mins[0] = amount },
                Some('b') => if amount > mins[1] { mins[1] = amount },
                Some('g') => if amount > mins[2] { mins[2] = amount },
                _ => panic!("color should follow a number")
            }
        }

        sum += mins[0] * mins[1] * mins[2];
    }

    println!("{sum}");
}
