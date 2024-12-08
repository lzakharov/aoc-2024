use std::collections::{HashMap, HashSet};
use std::io::BufRead;
use std::{fs::File, io};

fn main() {
    let file = File::open("input/day08.txt").unwrap();

    let mut n = 0;
    let mut m = 0;
    let mut antennas: HashMap<char, Vec<(isize, isize)>> = HashMap::new();

    io::BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .enumerate()
        .for_each(|(i, s)| {
            let chars: Vec<char> = s.chars().collect();
            n = (i + 1) as isize;
            m = chars.len() as isize;
            for (j, &c) in chars.iter().enumerate() {
                if c == '.' {
                    continue;
                }
                antennas
                    .entry(c)
                    .or_insert(Vec::new())
                    .push((i as isize, j as isize));
            }
        });

    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();

    for points in antennas.values() {
        for (i, &x) in points.iter().enumerate() {
            for &y in points[(i + 1)..].iter() {
                let dx = y.0 - x.0;
                let dy = y.1 - x.1;

                let p = (x.0 - dx, x.1 - dy);
                if p.0 >= 0 && p.0 < n && p.1 >= 0 && p.1 < m {
                    antinodes.insert(p);
                }

                let p = (y.0 + dx, y.1 + dy);
                if p.0 >= 0 && p.0 < n && p.1 >= 0 && p.1 < m {
                    antinodes.insert(p);
                }
            }
        }
    }

    println!("{}", antinodes.len());
}
