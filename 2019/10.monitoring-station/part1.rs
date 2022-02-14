
use std::io;
use std::io::prelude::*;
use std::collections::*;

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    return a;
}

fn reduce(frac: (i64, i64)) -> (i64, i64) {
    let div = gcd(frac.0.abs(), frac.1.abs());
    return (frac.0 / div, frac.1 / div);
}

fn count_visible(from: (i64, i64), all: &Vec<(i64, i64)>) -> i64 {
    let mut angles = HashSet::new();
    for p in all {
        if *p != from {
            angles.insert(reduce((p.0 - from.0, p.1 - from.1)));
        }
    }
    return angles.len() as i64;
}

fn main() {
    let stdin = io::stdin();
    let mut points = Vec::new();
    for (i, l) in stdin.lock().lines().enumerate() {
        for (j, c) in l.unwrap().chars().enumerate() {
            if c == '#' {
                points.push((i as i64, j as i64));
            }
        }
    }
    println!("Result: {}", points.iter().map(|x| count_visible(*x, &points)).max().unwrap());
}

