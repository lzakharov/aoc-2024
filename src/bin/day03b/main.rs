use std::fs;

use regex::Regex;

fn main() {
    let s = fs::read_to_string("input/day03.txt").unwrap();

    let re = Regex::new(r"mul\((?P<x>\d{1,3}),(?P<y>\d{1,3})\)").unwrap();

    let mut result = 0;
    let mut start = 0;
    let mut enabled = true;

    while let Some(i) = s[start..].find(if enabled { "don't()" } else { "do()" }) {
        if enabled {
            result += mul(&s[start..start + i], &re);
        }
        start = start + i;
        enabled = !enabled;
    }
    if enabled {
        result += mul(&s[start..], &re);
    }

    println!("{result}");
}

fn mul(s: &str, re: &Regex) -> i32 {
    re.captures_iter(s)
        .map(|c| c.extract())
        .map(|(_, [x, y])| x.parse::<i32>().unwrap() * y.parse::<i32>().unwrap())
        .sum::<i32>()
}
