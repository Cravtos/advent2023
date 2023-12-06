fn parse_line(s: &str) -> Vec<u64> {
    s.split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse::<u64>())
        .map(Result::unwrap)
        .collect()
}

fn main() {
    let mut input = include_str!("../input.txt").split('\n');
    let limits = parse_line(input.next().unwrap());
    let records = parse_line(input.next().unwrap());

    let sum: usize = limits
        .iter()
        .zip(records)
        .map(|(&limit, record)| {
            (1..limit)
                .map(|charge_time| (limit - charge_time) * charge_time)
                .filter(|&distance| distance > record)
                .count()
        })
        .product();
    println!("{sum}");
}
