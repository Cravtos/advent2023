use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashSet;

fn main() {
    let file = File::open("input.txt").expect("file input.txt should exist");
    let lines = io::BufReader::new(file).lines();

    let mut score = 0;

    for line in lines {
        let line = line.expect("line should be readable");
        let mut iter = line.split_whitespace().skip(2);
        
        let mut winning_numbers: HashSet<u32> = HashSet::new();
        while let Some(value) = iter.next() {
            if value == "|" {
                break;
            }

            let winning_number = value.parse::<u32>().expect("should be able to parse a number");
            winning_numbers.insert(winning_number);
        }

        let mut actual_numbers: HashSet<u32> = HashSet::new();
        while let Some(value) = iter.next() {
            if value == "|" {
                break;
            }

            let actual_number = value.parse::<u32>().expect("should be able to parse a number");
            actual_numbers.insert(actual_number);
        }

        let won_numbers = winning_numbers.intersection(&actual_numbers).collect::<Vec<&u32>>().len();
        if won_numbers > 0 {
            score += 1 << (won_numbers - 1); 
        }
    }

    println!("{score}");
}
