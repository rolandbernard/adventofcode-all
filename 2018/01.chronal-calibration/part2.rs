
use std::io;
use std::io::prelude::*;
use std::collections::*;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|x| x.unwrap()).collect::<Vec<String>>();
    let mut seen = HashSet::new();
    let mut sum = 0;
    'outer: loop {
        for line in &lines {
            if seen.contains(&sum) {
                break 'outer;
            } else {
                seen.insert(sum);
                sum += line.parse::<i64>().unwrap();
            }
        }
    }
    println!("Result: {}", sum);
}
