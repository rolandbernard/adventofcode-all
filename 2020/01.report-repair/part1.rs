
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
        let m = 2020 - n;
        if seen.contains(&m) {
            println!("Result: {}", m * n);
            break;
        }
        seen.insert(n);
    }
}

