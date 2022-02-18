
use std::io;
use std::io::prelude::*;
use std::collections::*;

fn main() {
    let stdin = io::stdin();
    let nums = stdin.lock().lines()
        .map(|x| x.unwrap().parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let mut seen = HashSet::new();
    for n in nums {
        for m in 0..(2020 - n) {
            let k = 2020 - n - m;
            if seen.contains(&k) && seen.contains(&m) {
                println!("Result: {}", m * n * k);
                return;
            }
        }
        seen.insert(n);
    }
}

