
use std::io;
use std::io::prelude::*;
use std::collections::*;

fn main() {
    let stdin = io::stdin();
    let mut answers = HashSet::new();
    let mut count = 0;
    for l in stdin.lock().lines() {
        let line = l.unwrap();
        if line == "" {
            count += answers.len();
            answers.clear();
        } else {
            for c in line.chars() {
                answers.insert(c);
            }
        }
    }
    count += answers.len();
    println!("Result: {}", count);
}

