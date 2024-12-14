use std::fs::File;
use std::io::{BufReader, Read};

fn main() {
    let file = File::open("input/day09.txt").unwrap();
    let mut xs: Vec<u8> = BufReader::new(file)
        .bytes()
        .map(|b| b.unwrap() - ('0' as u8))
        .collect();

    let mut result = 0;
    let mut i = 0;
    let mut j = xs.len() - 1;
    let mut cur = 0;

    while i <= j {
        if i % 2 == 0 {
            result += (i / 2) * sum(cur, cur + xs[i] as usize);
            cur += xs[i] as usize;
            i += 1;
            continue;
        }

        if xs[i] > xs[j] {
            result += (j / 2) * sum(cur, cur + xs[j] as usize);
            cur += xs[j] as usize;
            xs[i] -= xs[j];
            j -= 2;
            continue;
        }

        result += (j / 2) * sum(cur, cur + xs[i] as usize);
        cur += xs[i] as usize;
        if xs[i] < xs[j] {
            xs[j] -= xs[i];
        } else {
            j -= 2;
        }
        i += 1;
    }

    println!("{result}");
}

fn sum(n: usize, m: usize) -> usize {
    if n >= m {
        return 0;
    }
    (n + m - 1) * (m - n) / 2
}
