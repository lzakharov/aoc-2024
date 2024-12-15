use regex::Regex;
use std::io::{BufRead, BufReader};

const N: isize = 101;
const M: isize = 103;
const T: isize = 100;

fn main() {
    let file = std::fs::File::open("input/day14.txt").unwrap();
    let re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();

    let mut top_left = 0;
    let mut top_right = 0;
    let mut bottom_left = 0;
    let mut bottom_right = 0;

    BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .for_each(|line| {
            let (_, captures) = re.captures(&line).unwrap().extract();
            let [x, y, vx, vy]: [isize; 4] = captures.map(|s| s.parse().unwrap());

            let mut x = (x + vx * T) % N;
            let mut y = (y + vy * T) % M;
            if x < 0 {
                x += N;
            }
            if y < 0 {
                y += M;
            }

            if x < N / 2 && y < M / 2 {
                top_left += 1;
            }
            if x > N / 2 && y < M / 2 {
                top_right += 1;
            }
            if x < N / 2 && y > M / 2 {
                bottom_left += 1;
            }
            if x > N / 2 && y > M / 2 {
                bottom_right += 1;
            }
        });

    println!("{}", top_left * top_right * bottom_left * bottom_right);
}
