type MapRange = Vec<u64>;
type Map = Vec<MapRange>;
type Almanac = Vec<Map>;

fn intersect(f: &[u64; 2], s: &[u64; 2]) -> Option<[u64; 2]> {
    if f[0] > s[1] || s[0] > f[1] {
        return None;
    }

    Some([f[0].max(s[0]), f[1].min(s[1])])
}

fn remove_part(full: &[u64; 2], part: &[u64; 2]) -> Vec<[u64; 2]> {
    let mut parts: Vec<[u64; 2]> = Vec::new();

    if full[0] < part[0] {
        parts.push([full[0], part[0] - 1]);
    }

    if part[1] < full[1] {
        parts.push([part[1] + 1, full[1]])
    }

    parts
}

fn dest(from: &[u64; 2], map: &Map) -> Vec<[u64; 2]> {
    // check if src range intersects with any range with map and write down intersections
    let mut destinations: Vec<[u64; 2]> = Vec::new();
    let mut untouched: Vec<[u64; 2]> = vec![*from];
    for range in map {
        let dst = [range[0], range[0] + range[2] - 1];
        let src = [range[1], range[1] + range[2] - 1];

        if let Some(intr_src) = intersect(from, &src) {
            let intr_dst = [intr_src[0] - src[0] + dst[0], intr_src[1] - src[0] + dst[0]];
            destinations.push(intr_dst);
        }

        let mut new_untouched: Vec<[u64; 2]> = Vec::new();
        for unt_src in untouched.iter() {
            if let Some(intr) = intersect(unt_src, &src) {
                let mut untouched_parts = remove_part(unt_src, &intr);
                new_untouched.append(&mut untouched_parts);
            } else {
                new_untouched.push(*unt_src);
            }
        }
        untouched = new_untouched;
    }

    destinations.append(&mut untouched);
    destinations
}

fn parse_seeds(s: &str) -> Vec<u64> {
    s.split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse::<u64>())
        .map(Result::unwrap)
        .collect()
}

fn parse_map(s: &str) -> Map {
    s.split('\n')
        .skip(1)
        .map(str::split_whitespace)
        .map(|line| {
            line.map(|n| n.parse::<u64>())
                .map(Result::unwrap)
                .collect::<MapRange>()
        })
        .collect::<Map>()
}

fn main() {
    let mut almanac = include_str!("../input.txt").split("\n\n");
    let seeds = parse_seeds(almanac.next().unwrap());
    let almanac: Almanac = almanac.map(parse_map).collect::<Almanac>();

    let mut seeds: Vec<[u64; 2]> = seeds
        .chunks_exact(2)
        .map(|x| [x[0], x[0] + x[1] - 1])
        .collect();
    for map in almanac {
        let mut new_seeds: Vec<[u64; 2]> = Vec::new();

        for seed in seeds.iter() {
            new_seeds.append(&mut dest(seed, &map));
        }

        seeds = new_seeds;
    }

    let mut min = seeds[0][0];
    for seed in seeds {
        if seed[0] < min {
            min = seed[0];
        }
    }

    println!("{min}");
}
