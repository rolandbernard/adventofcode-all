
use std::io;
use std::io::prelude::*;
use std::collections::*;

fn main() {
    let stdin = io::stdin();
    let mut points = HashSet::new();
    let mut width = 0;
    let mut height = 0;
    for (i, l) in stdin.lock().lines().enumerate() {
        let line = l.unwrap();
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                points.insert((i, j));
            }
            width = j + 1;
        }
        height = i + 1;
    }
    let mut count = 0;
    let mut pos = (0, 0);
    while pos.0 <= height {
        if points.contains(&pos) {
            count += 1;
        }
        pos.0 += 1;
        pos.1 = (pos.1 + 3) % width;
    }
    println!("Result: {}", count);
}

