
use std::io;
use std::io::prelude::*;

fn seat_id(pass: &str) -> usize {
    let row = pass.chars().take(7)
        .map(|c| if c == 'B' { 1 } else { 0 })
        .fold(0, |a, x| 2*a + x);
    let col = pass.chars().skip(7).take(3)
        .map(|c| if c == 'R' { 1 } else { 0 })
        .fold(0, |a, x| 2*a + x);
    return 8 * row + col;
}

fn main() {
    let stdin = io::stdin();
    let ids = stdin.lock().lines()
        .map(|l| seat_id(&l.unwrap()))
        .collect::<Vec<_>>();
    let min = *ids.iter().min().unwrap();
    let max = *ids.iter().max().unwrap();
    let mut maybe = vec![true; max - min + 1];
    for id in ids {
        maybe[id - min] = false;
    }
    println!("Result: {}", min + maybe.iter().position(|&x| x).unwrap());
}

