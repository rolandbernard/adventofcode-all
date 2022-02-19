
use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let numbers = stdin.lock().lines().next().unwrap().unwrap()
        .split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
    let mut round = 1;
    let mut seen = Vec::new();
    let mut last = 0;
    while round <= numbers.len() {
        if round != 1 {
            if last >= seen.len() {
                seen.resize(last + 1, 0);
            }
            seen[last] = round;
        }
        last = numbers[round - 1];
        round += 1;
    }
    while round <= 30_000_000 {
        let next;
        if last >= seen.len() || seen[last] == 0 {
            next = 0;
        } else {
            next = round - seen[last];
        }
        if last >= seen.len() {
            seen.resize(last + 1, 0);
        }
        seen[last] = round;
        last = next;
        round += 1;
    }
    println!("Result: {}", last);
}

