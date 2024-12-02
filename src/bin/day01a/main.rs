use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter;

fn main() {
    let file = File::open("input/day01.txt").unwrap();
    let reader = BufReader::new(file);

    let (mut xs, mut ys): (Vec<i32>, Vec<i32>) = reader
        .lines()
        .map(|line| {
            let parsed: Vec<i32> = line
                .unwrap()
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            (parsed[0], parsed[1])
        })
        .unzip();

    xs.sort();
    ys.sort();

    let result: i32 = iter::zip(xs, ys).map(|(x, y)| (x - y).abs()).sum();
    println!("{result}");
}
