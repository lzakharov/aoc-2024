use std::io::BufRead;
use std::{fs::File, io};

fn main() {
    let file = File::open("input/day04.txt").unwrap();
    let reader = io::BufReader::new(file);

    let p: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let n = p.len();
    let m = p[0].len();
    let mut result = 0;

    for i in 1..n - 1 {
        for j in 1..m - 1 {
            if p[i][j] != 'A' {
                continue;
            }
            let xs = [
                p[i - 1][j - 1],
                p[i - 1][j + 1],
                p[i + 1][j + 1],
                p[i + 1][j - 1],
            ];
            result += match xs {
                ['M', 'M', 'S', 'S'] => 1,
                ['M', 'S', 'S', 'M'] => 1,
                ['S', 'M', 'M', 'S'] => 1,
                ['S', 'S', 'M', 'M'] => 1,
                _ => 0,
            }
        }
    }

    println!("{result}");
}
