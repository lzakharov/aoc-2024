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
                let mut sides = 0;
                walk(&map, &mut visited, (i, j), &mut area, &mut sides);
                result += area * sides;
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
    sides: &mut usize,
) {
    visited[i][j] = true;
    *area += 1;

    let n = map.len();
    let m = map[0].len();
    let x = map[i][j];
    if (i == 0 || map[i - 1][j] != x) && (j == 0 || map[i][j - 1] != x) {
        *sides += 1;
    }
    if (i == 0 || map[i - 1][j] != x) && (j == m - 1 || map[i][j + 1] != x) {
        *sides += 1;
    }
    if (i == n - 1 || map[i + 1][j] != x) && (j == 0 || map[i][j - 1] != x) {
        *sides += 1;
    }
    if (i == n - 1 || map[i + 1][j] != x) && (j == m - 1 || map[i][j + 1] != x) {
        *sides += 1;
    }
    if i > 0 && j < m - 1 && x == map[i - 1][j] && x == map[i][j + 1] && x != map[i - 1][j + 1] {
        *sides += 1;
    }
    if i > 0 && j > 0 && x == map[i - 1][j] && x == map[i][j - 1] && x != map[i - 1][j - 1] {
        *sides += 1;
    }
    if i < n - 1 && j > 0 && x == map[i + 1][j] && x == map[i][j - 1] && x != map[i + 1][j - 1] {
        *sides += 1;
    }
    if i < n - 1 && j < m - 1 && x == map[i + 1][j] && x == map[i][j + 1] && x != map[i + 1][j + 1]
    {
        *sides += 1;
    }

    if i > 0 && map[i - 1][j] == map[i][j] && !visited[i - 1][j] {
        walk(map, visited, (i - 1, j), area, sides);
    }
    if i < n - 1 && map[i + 1][j] == map[i][j] && !visited[i + 1][j] {
        walk(map, visited, (i + 1, j), area, sides);
    }
    if j > 0 && map[i][j - 1] == map[i][j] && !visited[i][j - 1] {
        walk(map, visited, (i, j - 1), area, sides);
    }
    if j < m - 1 && map[i][j + 1] == map[i][j] && !visited[i][j + 1] {
        walk(map, visited, (i, j + 1), area, sides);
    }
}
