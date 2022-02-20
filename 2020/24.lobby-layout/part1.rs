
use std::io;
use std::io::prelude::*;
use std::collections::*;

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
    println!("Result: {}", black.len());
}

