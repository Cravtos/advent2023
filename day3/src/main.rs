use std::fs::File;
use std::io::{self, BufRead};

fn get_number(s: &mut str, idx: usize) -> u32 {
    if s.chars().nth(idx).is_some_and(|ch| !ch.is_digit(10)) {
        return 0;
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
        Ok(n) => n,
        Err(_) => 0,
    }
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
        .filter(|(_, ch)| *ch != '.' && !ch.is_digit(10))
        .map(|(idx, _)| idx)
        .collect();
    for idx in idx_iter {
        sum += get_number(&mut buf[0], idx - 1) + get_number(&mut buf[0], idx + 1);
        let mid = get_number(&mut buf[1], idx);
        sum += if mid == 0 {
            get_number(&mut buf[1], idx - 1) + get_number(&mut buf[1], idx + 1)
        } else {
            mid
        };
    }

    // handle the middle
    loop {
        let idx_iter: Vec<usize> = buf[1]
            .char_indices()
            .filter(|(_, ch)| *ch != '.' && !ch.is_digit(10))
            .map(|(idx, _)| idx)
            .collect();
        for idx in idx_iter {
            sum += get_number(&mut buf[1], idx - 1) + get_number(&mut buf[1], idx + 1);

            let mid = get_number(&mut buf[0], idx);
            sum += if mid == 0 {
                get_number(&mut buf[0], idx - 1) + get_number(&mut buf[0], idx + 1)
            } else {
                mid
            };

            let mid = get_number(&mut buf[2], idx);
            sum += if mid == 0 {
                get_number(&mut buf[2], idx - 1) + get_number(&mut buf[2], idx + 1)
            } else {
                mid
            };
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
    let idx_iter: Vec<usize> = buf[1]
        .char_indices()
        .filter(|(_, ch)| *ch != '.' && !ch.is_digit(10))
        .map(|(idx, _)| idx)
        .collect();
    for idx in idx_iter {
        sum += get_number(&mut buf[2], idx - 1) + get_number(&mut buf[2], idx + 1);

        let mid = get_number(&mut buf[2], idx);
        sum += if mid == 0 {
            get_number(&mut buf[1], idx - 1) + get_number(&mut buf[1], idx + 1)
        } else {
            mid
        };
    }

    println!("{sum}");
}
