
use std::io;
use std::io::prelude::*;
use std::collections::*;

fn flip(old: HashSet<(i64, i64)>) -> HashSet<(i64, i64)> {
    let mut to_consider = old.clone();
    for &(i, j) in &old {
        for di in -2..=2 as i64 {
            for dj in -1..=1 as i64 {
                if di.abs() + dj.abs() == 2 {
                    to_consider.insert((i + di, j + dj));
                }
            }
        }
    }
    let mut new = HashSet::new();
    for (i, j) in to_consider {
        let mut neigh = 0;
        for di in -2..=2 as i64 {
            for dj in -1..=1 as i64 {
                if di.abs() + dj.abs() == 2 && old.contains(&(i + di, j + dj)) {
                    neigh += 1;
                }
            }
        }
        if (old.contains(&(i, j)) && neigh == 1) || neigh == 2 {
            new.insert((i, j));
        }
    }
    return new;
}

fn main() {
    let stdin = io::stdin();
    let mut black = HashSet::new();
    for line in stdin.lock().lines().map(|x| x.unwrap()) {
        let mut pos = (0, 0);
        let mut last = ' ';
        for c in line.chars() {
            match (last, c) {
                ('n', 'e') => pos = (pos.0 - 1, pos.1 - 1),
                ('n', 'w') => pos = (pos.0 + 1, pos.1 - 1),
                ('s', 'e') => pos = (pos.0 - 1, pos.1 + 1),
                ('s', 'w') => pos = (pos.0 + 1, pos.1 + 1),
                (_, 'e') => pos = (pos.0 - 2, pos.1),
                (_, 'w') => pos = (pos.0 + 2, pos.1),
                _ => {},
            }
            last = c;
        }
        if black.contains(&pos) {
            black.remove(&pos);
        } else {
            black.insert(pos);
        }
    }
    for _ in 0..100 {
        black = flip(black);
    }
    println!("Result: {}", black.len());
}

