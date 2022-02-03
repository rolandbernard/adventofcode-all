
use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let mut two = 0;
    let mut three = 0;
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        let mut counts = [0; 256];
        for c in l.bytes() {
            counts[c as usize] += 1;
        }
        if counts.contains(&2) {
            two += 1;
        }
        if counts.contains(&3) {
            three += 1;
        }
    }
    println!("Result: {}", two * three);
}
