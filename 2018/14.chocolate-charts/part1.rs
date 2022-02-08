
use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let n = stdin.lock().lines().next().unwrap().unwrap()
        .parse::<usize>().unwrap();
    let mut pos = (0, 1);
    let mut rec = vec![3, 7];
    while rec.len() < n + 10 {
        let sum = rec[pos.0] + rec[pos.1];
        if sum >= 10 {
            rec.push(sum / 10);
        }
        rec.push(sum % 10);
        pos.0 = (pos.0 + 1 + rec[pos.0]) % rec.len();
        pos.1 = (pos.1 + 1 + rec[pos.1]) % rec.len();
    }
    print!("Result: ");
    for i in 0..10 {
        print!("{}", rec[n + i]);
    }
    println!();
}

