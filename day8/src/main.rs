use std::collections::HashMap;

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

    let mut count: usize = 0;
    let mut poses = map.keys().filter(|node| node.chars().last().is_some_and(|c| c == 'A')).copied().collect::<Vec<&str>>();
    for insn in insns.chars().cycle() {
        if poses.iter().all(|pos| pos.chars().last().is_some_and(|c| c == 'Z')) {
            break;
        }

        poses.iter_mut().for_each(|pos| {
            let mut directions = *map.get(pos).unwrap();
            match insn {
                'L' => *pos = &mut directions.0,
                'R' => *pos = &mut directions.1,
                _ => unreachable!(),
            }
        });

        count += 1;
    }

    println!("{count}");
}
