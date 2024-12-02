use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input/day02.txt").unwrap();
    let reader = BufReader::new(file);
    let mut result = 0;

    reader.lines().for_each(|line| {
        let xs: Vec<i32> = line
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        for i in 0..xs.len() {
            let xs = [&xs[..i], &xs[i + 1..]].concat();
            if check(&xs) {
                result += 1;
                return;
            }
        }
    });

    println!("{result}");
}

fn check(xs: &[i32]) -> bool {
    let signum = (xs[1] - xs[0]).signum();
    for d in xs.windows(2).map(|w| w[1] - w[0]) {
        if d.signum() != signum || d.abs() < 1 || d.abs() > 3 {
            return false;
        }
    }
    return true;
}
