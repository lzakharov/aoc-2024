use regex::Regex;
use std::{
    cmp,
    io::{BufRead, BufReader},
};

fn main() {
    let file = std::fs::File::open("input/day13.txt").unwrap();
    let mut lines = BufReader::new(file).lines().map(Result::unwrap);

    let button_re = Regex::new(r"Button .: X\+(\d+), Y\+(\d+)").unwrap();
    let prize_re = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

    let mut result = 0;

    while let (Some(a), Some(b), Some(prize)) = (lines.next(), lines.next(), lines.next()) {
        let (_, [ax, ay]) = button_re.captures(&a).unwrap().extract();
        let (_, [bx, by]) = button_re.captures(&b).unwrap().extract();
        let (_, [x, y]) = prize_re.captures(&prize).unwrap().extract();
        let ax: usize = ax.parse().unwrap();
        let ay: usize = ay.parse().unwrap();
        let bx: usize = bx.parse().unwrap();
        let by: usize = by.parse().unwrap();
        let x: usize = x.parse().unwrap();
        let y: usize = y.parse().unwrap();

        let num = (x * by) as isize - (bx * y) as isize;
        let den = (ax * by) as isize - (bx * ay) as isize;

        if den == 0 {
            result += cmp::min(3 * (x / ax), x / bx);
        } else if num % den != 0 {
        } else {
            let n = (num / den) as usize;
            let m = (x - n * ax) / bx;
            result += 3 * n + m;
        }

        lines.next();
    }

    println!("{result}");
}
