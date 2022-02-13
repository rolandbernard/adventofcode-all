
use std::io;
use std::io::prelude::*;

fn fuel(mass: i64) -> i64 {
    let f = (mass / 3 - 2).max(0);
    if f != 0 {
        return f + fuel(f);
    } else {
        return f;
    }
}

fn main() {
    let stdin = io::stdin();
    let mass = stdin.lock().lines()
        .map(|x| x.unwrap().parse::<i64>().unwrap())
        .map(fuel).sum::<i64>();
    println!("Result: {}", mass);
}

