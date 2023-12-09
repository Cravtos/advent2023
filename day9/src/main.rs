fn fact(n: usize) -> usize {
    (1..=n).product()
}

fn comb(n: usize, r: usize) -> usize {
    (n - r + 1..=n).product::<usize>() / fact(r)
}

fn extrapolate(hist: Vec<i64>) -> i64 {
    let mut steps = 0;
    let mut diffs = hist.clone();
    while !diffs.iter().all(|&x| x == 0) {
        diffs = diffs.windows(2).map(|w| w[1] - w[0]).collect::<Vec<i64>>();
        steps += 1;
    }

    let mut guess: i64 = *hist.last().unwrap();
    for i in 1..steps {
        let mut part =
            comb(steps - 1, i) as i64 * (hist[hist.len() - i] - hist[hist.len() - i - 1]);
        if i % 2 == 0 {
            part = -part;
        }
        guess += part;
    }

    guess
}

fn main() {
    let sum: i64 = include_str!("../input.txt")
        .lines()
        .map(|line| {
            let hist = line
                .split(' ')
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            extrapolate(hist)
        })
        .sum();

    println!("{sum}");
}
