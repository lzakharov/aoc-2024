use std::io::BufRead;
use std::{fs::File, io};

fn main() {
    let file = File::open("input/day07.txt").unwrap();

    let result: usize = io::BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .map(|s| {
            let (n, xs) = s.split_at(s.find(':').unwrap());
            let n = n.parse::<usize>().unwrap();
            let xs = xs[2..]
                .split_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            if check(n, &xs[1..], xs[0]) {
                n
            } else {
                0
            }
        })
        .sum();

    println!("{result}");
}

fn check(n: usize, xs: &[usize], acc: usize) -> bool {
    if xs.is_empty() {
        return acc == n;
    }

    let sum = acc + xs[0];
    let mul = acc * xs[0];
    (sum <= n && check(n, &xs[1..], sum)) || (mul <= n && check(n, &xs[1..], mul))
}
