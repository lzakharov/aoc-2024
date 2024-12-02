use std::fs::File;
use std::io::{BufRead, BufReader};
use std::usize;

fn main() {
    let file = File::open("input/day01.txt").unwrap();
    let reader = BufReader::new(file);

    let mut ys = [0; 100000];

    let xs: Vec<usize> = reader
        .lines()
        .map(|line| {
            let parsed: Vec<usize> = line
                .unwrap()
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            ys[parsed[1]] += 1;
            parsed[0]
        })
        .collect();

    let result: usize = xs.iter().map(|&x| x * ys[x]).sum();
    println!("{result}");
}
