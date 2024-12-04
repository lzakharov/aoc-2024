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

    for i in 0..n {
        for j in 0..m {
            if p[i][j] != 'X' {
                continue;
            }

            if j > 2 {
                result += check(&p, [i, j - 1, i, j - 2, i, j - 3]);
            }
            if i > 2 && j > 2 {
                result += check(&p, [i - 1, j - 1, i - 2, j - 2, i - 3, j - 3])
            }
            if i > 2 {
                result += check(&p, [i - 1, j, i - 2, j, i - 3, j]);
            }
            if i > 2 && j < m - 3 {
                result += check(&p, [i - 1, j + 1, i - 2, j + 2, i - 3, j + 3]);
            }
            if j < m - 3 {
                result += check(&p, [i, j + 1, i, j + 2, i, j + 3]);
            }
            if i < n - 3 && j < m - 3 {
                result += check(&p, [i + 1, j + 1, i + 2, j + 2, i + 3, j + 3]);
            }
            if i < n - 3 {
                result += check(&p, [i + 1, j, i + 2, j, i + 3, j]);
            }
            if i < n - 3 && j > 2 {
                result += check(&p, [i + 1, j - 1, i + 2, j - 2, i + 3, j - 3]);
            }
        }
    }

    println!("{result}");
}

fn check(puzzle: &Vec<Vec<char>>, xs: [usize; 6]) -> i32 {
    if puzzle[xs[0]][xs[1]] == 'M' && puzzle[xs[2]][xs[3]] == 'A' && puzzle[xs[4]][xs[5]] == 'S' {
        return 1;
    }
    0
}
