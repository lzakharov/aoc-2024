use std::fs;

use regex::Regex;

fn main() {
    let s = fs::read_to_string("input/day03.txt").unwrap();

    let re = Regex::new(r"mul\((?P<x>\d{1,3}),(?P<y>\d{1,3})\)").unwrap();
    let result: i32 = re
        .captures_iter(&s)
        .map(|c| c.extract())
        .map(|(_, [x, y])| x.parse::<i32>().unwrap() * y.parse::<i32>().unwrap())
        .sum();

    println!("{result}");
}
