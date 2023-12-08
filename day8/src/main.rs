use std::collections::HashMap;

fn gcd(mut n: usize, mut m: usize) -> usize {
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn main() {
    let mut input = include_str!("../input.txt").split("\n\n");
    let insns = input.next().unwrap();

    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    let lines = input.next().unwrap().lines();

    for line in lines {
        let mut line = line.split(" = ");
        let node = line.next().unwrap();
        let mut directions = line
            .next()
            .unwrap()
            .strip_prefix('(')
            .unwrap()
            .strip_suffix(')')
            .unwrap()
            .split(", ");
        let directions = (directions.next().unwrap(), directions.next().unwrap());
        map.insert(node, directions);
    }

    let mut poses = map
        .keys()
        .filter(|node| node.chars().last().is_some_and(|c| c == 'A'))
        .copied()
        .collect::<Vec<&str>>();
    let mut periods: Vec<Option<usize>> = vec![None; poses.len()];
    for (count, insn) in insns.chars().cycle().enumerate() {
        if periods.iter().all(Option::is_some) {
            break;
        }

        poses
            .iter()
            .enumerate()
            .filter(|(_, pos)| pos.chars().last().is_some_and(|c| c == 'Z'))
            .for_each(|(i, _)| {
                if let Some(period) = periods[i] {
                    if period * 2 != count {
                        panic!("inconsist period");
                    }
                } else {
                    periods[i] = Some(count);
                }
            });

        poses.iter_mut().for_each(|pos| {
            let mut directions = *map.get(pos).unwrap();
            match insn {
                'L' => *pos = &mut directions.0,
                'R' => *pos = &mut directions.1,
                _ => unreachable!(),
            }
        });
    }

    let steps = periods.iter().map(|p| p.unwrap()).reduce(lcm).unwrap();
    println!("{steps}");
}
