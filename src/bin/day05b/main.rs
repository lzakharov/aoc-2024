use std::io::BufRead;
use std::{fs::File, io};

fn main() {
    let file = File::open("input/day05.txt").unwrap();
    let mut lines = io::BufReader::new(file).lines().map(Result::unwrap);

    let mut rules = [[false; 100]; 100];

    lines
        .by_ref()
        .take_while(|s| !s.is_empty())
        .map(|s| {
            s.split("|")
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .for_each(|rule| {
            rules[rule[0]][rule[1]] = true;
        });

    let result: usize = lines
        .map(|s| {
            s.split(",")
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .map(|mut update| {
            let mut flag = false;

            for i in 0..update.len() {
                for j in i + 1..update.len() {
                    if rules[update[j]][update[i]] {
                        flag = true;
                        (update[i], update[j]) = (update[j], update[i]);
                    }
                }
            }

            if flag {
                update[update.len() / 2]
            } else {
                0
            }
        })
        .sum();

    println!("{result}");
}
