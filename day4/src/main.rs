use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashSet;

fn get_matching_amount(card: &str) -> usize {
    let mut iter = card.split_whitespace().skip(2);
        
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
        let actual_number = value.parse::<u32>().expect("should be able to parse a number");
        actual_numbers.insert(actual_number);
    }

    winning_numbers.intersection(&actual_numbers).collect::<Vec<&u32>>().len()
}

fn main() {
    let file = File::open("input.txt").expect("file input.txt should exist");
    let lines = io::BufReader::new(file).lines();

    let mut sum = 0;
    let mut copies_records: Vec<(usize, usize)> = vec![]; // (amount of copies, for next N cards)

    for line in lines {
        let copies: usize = copies_records.iter().map(|(copies, _)| copies).sum();
        sum += 1 + copies;

        // remove amount of copies records that are valid for 0 next cards
        copies_records = copies_records.iter().map(|(copies, n)| (*copies, n-1)).filter(|(_, n)| *n > 0).collect();
        
        let card = line.expect("line should be readable");
        let won_amount = get_matching_amount(&card);
        if won_amount > 0 {
            copies_records.push((1 + copies, won_amount));
        }
    }

    println!("{sum}");
}
