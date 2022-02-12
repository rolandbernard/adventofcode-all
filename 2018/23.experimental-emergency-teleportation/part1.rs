
use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let mut bots = Vec::new();
    for l in stdin.lock().lines() {
        let line = l.unwrap();
        let splt = line.split(|c| c == '<' || c == '>' || c == '=' || c == ',')
            .map(|x| x.parse::<i64>().ok()).collect::<Vec<Option<i64>>>();
        bots.push(((splt[2].unwrap(), splt[3].unwrap(), splt[4].unwrap()), (splt[7].unwrap())));
    }
    let mut m = bots[0];
    for &b in &bots {
        if b.1 > m.1 {
            m = b;
        }
    }
    let mut count = 0;
    for &b in &bots {
        if (b.0.0 - m.0.0).abs() + (b.0.1 - m.0.1).abs() + (b.0.2 - m.0.2).abs() <= m.1 {
            count += 1;
        }
    }
    println!("Result: {}", count);
}

