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

    let mut count = 0;
    let mut pos = "AAA";
    for insn in insns.chars().cycle() {
        if pos == "ZZZ" {
            break;
        }

        let directions = map.get(pos).unwrap();
        match insn {
            'L' => pos = directions.0,
            'R' => pos = directions.1,
            _ => unreachable!(),
        }

        count += 1;
    }

    println!("{count}");
}
