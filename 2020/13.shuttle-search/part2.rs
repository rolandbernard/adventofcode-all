
use std::io;
use std::io::prelude::*;

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    } else {
        return gcd(b, a % b);
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    return a * b / gcd(a, b);
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines()
        .map(|l| l.unwrap())
        .collect::<Vec<_>>();
    let buses = lines[1].split(",")
        .map(|x| x.parse::<i64>().unwrap_or(0))
        .collect::<Vec<_>>();
    let mut time = 0;
    let mut mult = 1;
    for (i, b) in buses.into_iter().enumerate() {
        if b != 0 {
            while b - 1 - (time % b) != i as i64 % b {
                time += mult;
            }
            mult = lcm(mult, b);
        }
    }
    println!("Result: {}", time + 1);
}

