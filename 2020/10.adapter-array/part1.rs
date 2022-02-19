
use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let mut nums = stdin.lock().lines()
        .map(|l| l.unwrap().parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    nums.sort();
    let mut counts = [0, 0, 0, 1];
    let mut last = 0;
    for x in nums {
        let dif = x - last;
        counts[dif] += 1;
        last = x;
    }
    println!("Result: {}", counts[1] * counts[3]);
}

