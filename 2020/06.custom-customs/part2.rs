
use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let mut answers = [0; 26];
    let mut persons = 0;
    let mut count = 0;
    for l in stdin.lock().lines() {
        let line = l.unwrap();
        if line == "" {
            count += answers.iter().filter(|&&x| x == persons).count();
            persons = 0;
            answers = [0; 26];
        } else {
            for c in line.chars() {
                answers[c.to_digit(36).unwrap() as usize - 10] += 1;
            }
            persons += 1;
        }
    }
    count += answers.iter().filter(|&&x| x == persons).count();
    println!("Result: {}", count);
}

