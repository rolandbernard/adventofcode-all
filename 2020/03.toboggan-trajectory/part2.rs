
use std::io;
use std::io::prelude::*;
use std::collections::*;

fn count_trees(slope: (usize, usize), size: (usize, usize), points: &HashSet<(usize, usize)>) -> usize {
    let mut count = 0;
    let mut pos = (0, 0);
    while pos.0 <= size.0 {
        if points.contains(&pos) {
            count += 1;
        }
        pos.0 += slope.0;
        pos.1 = (pos.1 + slope.1) % size.1;
    }
    return count;
}

fn main() {
    let stdin = io::stdin();
    let mut points = HashSet::new();
    let mut size = (0, 0);
    for (i, l) in stdin.lock().lines().enumerate() {
        let line = l.unwrap();
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                points.insert((i, j));
            }
            size.1 = j + 1;
        }
        size.0 = i + 1;
    }
    let mut res = 1;
    for slope in [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)] {
        res *= count_trees(slope, size, &points);
    }
    println!("Result: {}", res);
}

