
use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines()
        .map(|l| l.unwrap())
        .collect::<Vec<_>>();
    let arrive = lines[0].parse::<i64>().unwrap();
    let buses = lines[1].split(",")
        .filter(|&x| x != "x")
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let mut min = i64::MAX;
    let mut bus = 0;
    for b in buses {
        let wait = b - (arrive % b);
        if wait < min {
            bus = b;
            min = wait;
        }
    }
    println!("Result: {}", bus * min);
}

