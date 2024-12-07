use std::collections::HashSet;
use std::io::BufRead;
use std::{fs::File, io};

fn main() {
    let file = File::open("input/day06.txt").unwrap();
    let mut map: Vec<Vec<char>> = io::BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .map(|s| s.chars().collect())
        .collect();

    let n = map.len();
    let m = map[0].len();
    let mut start = (0, 0);
    for i in 0..n {
        for j in 0..m {
            if map[i][j] == '^' {
                start = (i, j);
            }
        }
    }

    let mut result = 0;

    // FIXME: slow solution (≈1m30s)
    for i in 0..n {
        for j in 0..m {
            if map[i][j] != '.' {
                continue;
            }
            map[i][j] = '#';

            let mut pos = start;
            let mut direction: (i32, i32) = (-1, 0);
            let mut seen: HashSet<((usize, usize), (i32, i32))> = HashSet::new();
            seen.insert((pos, direction));

            loop {
                let next = (pos.0 as i32 + direction.0, pos.1 as i32 + direction.1);
                if next.0 < 0 || next.0 >= n as i32 || next.1 < 0 || next.1 >= m as i32 {
                    break;
                }
                let next = (next.0 as usize, next.1 as usize);

                if map[next.0][next.1] == '#' {
                    direction = match direction {
                        (-1, 0) => (0, 1),
                        (0, 1) => (1, 0),
                        (1, 0) => (0, -1),
                        (0, -1) => (-1, 0),
                        _ => unreachable!(),
                    };
                    continue;
                }

                pos = next;
                if seen.contains(&(pos, direction)) {
                    result += 1;
                    break;
                }
                seen.insert((pos, direction));
            }

            map[i][j] = '.';
        }
    }

    println!("{result}");
}
