use std::fs::File;
use std::io::{self, BufRead};

fn cut_number(s: &mut str, idx: usize) -> Option<u32> {
    if s.chars().nth(idx).is_some_and(|ch| !ch.is_digit(10)) {
        return None;
    }

    let mut start = idx;
    while start > 0 && s.chars().nth(start - 1).is_some_and(|ch| ch.is_digit(10)) {
        start -= 1;
    }

    let mut end = idx;
    while s.chars().nth(end).is_some_and(|ch| ch.is_digit(10)) {
        end += 1;
    }

    let number: String = s.chars().skip(start).take(end - start).collect();
    let number = number.parse::<u32>();

    // remove read numbers to avoid duplicates
    let raw_str = unsafe { s.as_bytes_mut() };
    for i in start..end {
        raw_str[i] = b'.';
    }

    match number {
        Ok(n) => Some(n),
        Err(_) => None,
    }
}

fn append_to_vec_if_some(vec: &mut Vec<u32>, number: Option<u32>) -> bool {
    if let Some(n) = number {
        vec.push(n);
        return true;
    }
    return false;
}

fn main() {
    let file = File::open("input.txt").expect("file input.txt should exist");
    let mut lines = io::BufReader::new(file).lines();

    let mut sum = 0;

    let mut buf: Vec<String> = (&mut lines)
        .take(3)
        .map(|line| line.expect("should be able to read line"))
        .collect();

    // handle first line
    let idx_iter: Vec<usize> = buf[0]
        .char_indices()
        .filter(|(_, ch)| *ch == '*')
        .map(|(idx, _)| idx)
        .collect();
    for idx in idx_iter {
        let mut numbers: Vec<u32> = Vec::new();

        append_to_vec_if_some(&mut numbers, cut_number(&mut buf[0], idx - 1));
        append_to_vec_if_some(&mut numbers, cut_number(&mut buf[0], idx + 1));

        if !append_to_vec_if_some(&mut numbers, cut_number(&mut buf[1], idx)) {
            append_to_vec_if_some(&mut numbers, cut_number(&mut buf[1], idx-1));
            append_to_vec_if_some(&mut numbers, cut_number(&mut buf[1], idx+1));
        }

        if numbers.len() == 2 {
            sum += numbers[0] * numbers[1];
        }
    }

    // handle the middle
    loop {
        let idx_iter: Vec<usize> = buf[1]
            .char_indices()
            .filter(|(_, ch)| *ch == '*')
            .map(|(idx, _)| idx)
            .collect();
        for idx in idx_iter {
            let mut numbers: Vec<u32> = Vec::new();

            append_to_vec_if_some(&mut numbers, cut_number(&mut buf[1], idx - 1));
            append_to_vec_if_some(&mut numbers, cut_number(&mut buf[1], idx + 1));

            if !append_to_vec_if_some(&mut numbers, cut_number(&mut buf[0], idx)) {
                append_to_vec_if_some(&mut numbers, cut_number(&mut buf[0], idx-1));
                append_to_vec_if_some(&mut numbers, cut_number(&mut buf[0], idx+1));
            }

            if !append_to_vec_if_some(&mut numbers, cut_number(&mut buf[2], idx)) {
                append_to_vec_if_some(&mut numbers, cut_number(&mut buf[2], idx-1));
                append_to_vec_if_some(&mut numbers, cut_number(&mut buf[2], idx+1));
            }

            if numbers.len() == 2 {
                sum += numbers[0] * numbers[1];
            }
        }

        buf.swap(0, 1);
        buf.swap(1, 2);

        let line = lines.next();
        match line {
            Some(line) => buf[2] = line.expect("should be able to read line"),
            None => break,
        }
    }

    // handle last line
    let idx_iter: Vec<usize> = buf[2]
        .char_indices()
        .filter(|(_, ch)| *ch == '*')
        .map(|(idx, _)| idx)
        .collect();
    for idx in idx_iter {
        let mut numbers: Vec<u32> = Vec::new();

        append_to_vec_if_some(&mut numbers, cut_number(&mut buf[2], idx - 1));
        append_to_vec_if_some(&mut numbers, cut_number(&mut buf[2], idx + 1));

        if !append_to_vec_if_some(&mut numbers, cut_number(&mut buf[1], idx)) {
            append_to_vec_if_some(&mut numbers, cut_number(&mut buf[1], idx-1));
            append_to_vec_if_some(&mut numbers, cut_number(&mut buf[1], idx+1));
        }

        if numbers.len() == 2 {
            sum += numbers[0] * numbers[1];
        }
    }

    println!("{sum}");
}
