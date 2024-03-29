type Range = [u64; 2];
type MapRange = Vec<u64>;
type Map = Vec<MapRange>;
type Almanac = Vec<Map>;

fn intersect(f: &Range, s: &Range) -> Option<Range> {
    if f[0] > s[1] || s[0] > f[1] {
        return None;
    }

    Some([f[0].max(s[0]), f[1].min(s[1])])
}

fn remove_part(full: &Range, part: &Range) -> Vec<Range> {
    let mut parts: Vec<Range> = Vec::new();

    if full[0] < part[0] {
        parts.push([full[0], part[0] - 1]);
    }

    if part[1] < full[1] {
        parts.push([part[1] + 1, full[1]])
    }

    parts
}

fn dest(from: &Range, map: &Map) -> Vec<Range> {
    let mut destinations: Vec<Range> = Vec::new();
    let mut untouched: Vec<Range> = vec![*from];
    
    for range in map {
        let dst = [range[0], range[0] + range[2] - 1];
        let src = [range[1], range[1] + range[2] - 1];

        // check if src range intersects with any range with map and write down intersections
        if let Some(intr) = intersect(from, &src) {
            let intr_dst = [intr[0] - src[0] + dst[0], intr[1] - src[0] + dst[0]];
            destinations.push(intr_dst);
        }

        // if range does not intersect, keep it as is
        let mut new_untouched: Vec<Range> = Vec::new();
        for unt in untouched.iter() {
            if let Some(intr) = intersect(unt, &src) {
                let mut untouched_parts = remove_part(unt, &intr);
                new_untouched.append(&mut untouched_parts);
            } else {
                new_untouched.push(*unt);
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

    let mut seeds: Vec<Range> = seeds
        .chunks_exact(2)
        .map(|x| [x[0], x[0] + x[1] - 1])
        .collect();

    for map in almanac {
        let mut new_seeds: Vec<Range> = Vec::new();
        for seed in seeds.iter() {
            new_seeds.append(&mut dest(seed, &map));
        }
        seeds = new_seeds;
    }

    let min = seeds.iter().min_by(|x, y| x[0].cmp(&y[0])).unwrap()[0];
    println!("{min}");
}
