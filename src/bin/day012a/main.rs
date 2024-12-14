use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("input/day12.txt").unwrap();
    let map: Vec<Vec<char>> = BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .map(|line| line.chars().collect())
        .collect();

    let n = map.len();
    let m = map[0].len();
    let mut visited = vec![vec![false; m]; n];

    let mut result = 0;

    for i in 0..n {
        for j in 0..m {
            if !visited[i][j] {
                let mut area = 0;
                let mut perimiter = 0;
                walk(&map, &mut visited, (i, j), &mut area, &mut perimiter);
                result += area * perimiter;
            }
        }
    }

    println!("{result}");
}

fn walk(
    map: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    (i, j): (usize, usize),
    area: &mut usize,
    perimiter: &mut usize,
) {
    visited[i][j] = true;
    *area += 1;
    *perimiter += 4;

    if i > 0 {
        if map[i - 1][j] == map[i][j] {
            *perimiter -= 1;
            if !visited[i - 1][j] {
                walk(map, visited, (i - 1, j), area, perimiter);
            }
        }
    }
    if i < map.len() - 1 {
        if map[i + 1][j] == map[i][j] {
            *perimiter -= 1;
            if !visited[i + 1][j] {
                walk(map, visited, (i + 1, j), area, perimiter);
            }
        }
    }
    if j > 0 {
        if map[i][j - 1] == map[i][j] {
            *perimiter -= 1;
            if !visited[i][j - 1] {
                walk(map, visited, (i, j - 1), area, perimiter);
            }
        }
    }
    if j < map[0].len() - 1 {
        if map[i][j + 1] == map[i][j] {
            *perimiter -= 1;
            if !visited[i][j + 1] {
                walk(map, visited, (i, j + 1), area, perimiter);
            }
        }
    }
}
