
use std::io;
use std::io::prelude::*;
use std::collections::*;

fn main() {
    let stdin = io::stdin();
    let numbers = stdin.lock().lines().next().unwrap().unwrap()
        .split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
    let mut round = 0;
    let mut seen = HashMap::new();
    let mut last = 0;
    while round < numbers.len() {
        if round != 0 {
            seen.insert(last, round);
        }
        last = numbers[round];
        round += 1;
    }
    while round < 2020 {
        let next;
        if seen.contains_key(&last) {
            next = round - seen[&last];
        } else {
            next = 0;
        }
        seen.insert(last, round);
        last = next;
        round += 1;
    }
    println!("Result: {}", last);
}

