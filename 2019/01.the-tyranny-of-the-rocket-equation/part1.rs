
use std::io;
use std::io::prelude::*;

fn fuel(mass: i64) -> i64 {
    return (mass / 3 - 2).max(0);
}

fn main() {
    let stdin = io::stdin();
    let mass = stdin.lock().lines()
        .map(|x| x.unwrap().parse::<i64>().unwrap())
        .map(fuel).sum::<i64>();
    println!("Result: {}", mass);
}

