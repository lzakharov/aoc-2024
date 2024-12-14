use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input/day10.txt").unwrap();
    let map: Vec<Vec<u8>> = BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .map(|line| line.bytes().map(|c| c - ('0' as u8)).collect())
        .collect();

    let mut result = 0;

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == 0 {
                result += walk(&map, (i, j));
            }
        }
    }

    println!("{result}");
}

fn walk(map: &Vec<Vec<u8>>, pos: (usize, usize)) -> usize {
    if map[pos.0][pos.1] == 9 {
        return 1;
    }

    let mut result = 0;
    if pos.0 > 0 && map[pos.0 - 1][pos.1] == map[pos.0][pos.1] + 1 {
        result += walk(map, (pos.0 - 1, pos.1));
    }
    if pos.1 > 0 && map[pos.0][pos.1 - 1] == map[pos.0][pos.1] + 1 {
        result += walk(map, (pos.0, pos.1 - 1));
    }
    if pos.0 < map.len() - 1 && map[pos.0 + 1][pos.1] == map[pos.0][pos.1] + 1 {
        result += walk(map, (pos.0 + 1, pos.1));
    }
    if pos.1 < map[0].len() - 1 && map[pos.0][pos.1 + 1] == map[pos.0][pos.1] + 1 {
        result += walk(map, (pos.0, pos.1 + 1));
    }
    result
}
