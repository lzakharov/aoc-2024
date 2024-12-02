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

        let s = (xs[1] - xs[0]).signum();
        for d in xs.windows(2).map(|w| w[1] - w[0]) {
            if d.signum() != s || d.abs() < 1 || d.abs() > 3 {
                return;
            }
        }

        result += 1;
    });

    println!("{result}");
}
