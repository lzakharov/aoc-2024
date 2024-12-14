use std::fs::File;
use std::io::{BufReader, Read};

fn main() {
    let file = File::open("input/day09.txt").unwrap();
    let xs: Vec<u8> = BufReader::new(file)
        .bytes()
        .map(|b| b.unwrap() - ('0' as u8))
        .collect();
    let mut ys = xs.clone();

    let mut result = 0;
    let mut i = 0;
    let mut cur = 0;

    while i < xs.len() {
        if i % 2 == 0 {
            result += (i / 2) * sum(cur, cur + ys[i] as usize);
            cur += xs[i] as usize;
            i += 1;
            continue;
        }

        let mut j = xs.len() - 1;
        while j > i {
            if ys[j] > 0 && ys[i] >= ys[j] {
                result += (j / 2) * sum(cur, cur + ys[j] as usize);
                cur += ys[j] as usize;
                ys[i] -= ys[j];
                ys[j] = 0;
                j = xs.len() - 1;
                continue;
            }
            j -= 2;
        }
        if j < i {
            cur += ys[i] as usize;
            i += 1;
        }
    }

    println!("{result}");
}

fn sum(n: usize, m: usize) -> usize {
    if n >= m {
        return 0;
    }
    (n + m - 1) * (m - n) / 2
}
