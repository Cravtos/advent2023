fn main() {
    let mut input = include_str!("../input.txt").split('\n');
    let limit = input.next().unwrap().split(':').nth(1).unwrap().replace(' ', "").parse::<usize>().unwrap();
    let record = input.next().unwrap().split(':').nth(1).unwrap().replace(' ', "").parse::<usize>().unwrap();

    let sum = (1..limit)
                .map(|charge_time| (limit - charge_time) * charge_time)
                .filter(|&distance| distance > record)
                .count();
    println!("{sum}");
}
