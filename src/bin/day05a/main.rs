use std::io::BufRead;
use std::{fs::File, io};

fn main() {
    let file = File::open("input/day05.txt").unwrap();
    let mut lines = io::BufReader::new(file).lines().map(Result::unwrap);

    let mut rules = [(); 100].map(|_| Vec::new());

    lines
        .by_ref()
        .take_while(|s| !s.is_empty())
        .map(|s| {
            s.split("|")
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .for_each(|rule| {
            rules[rule[0]].push(rule[1]);
        });

    let result: usize = lines
        .map(|s| {
            s.split(",")
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .map(|update| {
            let mut seen = [false; 100];
            for &page in update.iter() {
                for &x in rules[page].iter() {
                    if seen[x] {
                        return 0;
                    }
                }
                seen[page] = true;
            }
            update[update.len() / 2]
        })
        .sum();

    println!("{result}");
}
