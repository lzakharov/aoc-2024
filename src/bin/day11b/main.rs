use std::collections::HashMap;

const N: usize = 75;

fn main() {
    let stones: Vec<usize> = std::fs::read_to_string("input/day11.txt")
        .unwrap()
        .split_whitespace()
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    let mut mem = HashMap::new();
    let result = stones
        .iter()
        .map(|&stone| f(stone, 0, &mut mem))
        .sum::<usize>();

    println!("{result}");
}

fn f(stone: usize, i: usize, mem: &mut HashMap<(usize, usize), usize>) -> usize {
    if i == N {
        return 1;
    }
    blink(stone)
        .iter()
        .map(|&stone| {
            if let Some(&n) = mem.get(&(stone, i)) {
                return n;
            }
            let n = f(stone, i + 1, mem);
            mem.insert((stone, i), n);
            n
        })
        .sum()
}

fn blink(stone: usize) -> Vec<usize> {
    if stone == 0 {
        return vec![1];
    }
    if stone == 1 {
        return vec![2024];
    }
    let s = stone.to_string();
    if s.len() % 2 == 0 {
        let (x, y) = s.split_at(s.len() / 2);
        let x = x.parse::<usize>().unwrap();
        let y = y.parse::<usize>().unwrap();
        return vec![x, y];
    }
    let x = 2024 * stone;
    return vec![x];
}
