type Range = Vec<u64>;
type Map = Vec<Range>;
type Almanac = Vec<Map>;

fn dest(src: u64, map: &Map) -> u64 {
    for range in map {
        if (range[1]..range[1] + range[2]).contains(&src) {
            return range[0] + (src - range[1]);
        }
    }

    src
}

fn parse_seeds(s: &str) -> Vec<u64> {
    s.split(":")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|n| u64::from_str_radix(n, 10))
        .map(Result::unwrap)
        .collect()
}

fn parse_map(s: &str) -> Map {
    s.split("\n")
        .skip(1)
        .map(str::split_whitespace)
        .map(|line| {
            line.map(|n| u64::from_str_radix(n, 10))
                .map(Result::unwrap)
                .collect::<Range>()
        })
        .collect::<Map>()
}

fn main() {
    let mut almanac = include_str!("../input.txt").split("\n\n");
    let seeds: Vec<u64> = parse_seeds(almanac.next().unwrap());
    let almanac: Almanac = almanac.map(parse_map).collect::<Almanac>();
    let locations: Vec<u64> = almanac.iter().fold(seeds, |acc, map| {
        acc.iter().map(|&x| dest(x, map)).collect::<Vec<u64>>()
    });
    let min = locations.iter().min().unwrap();
    println!("{min}");
}
