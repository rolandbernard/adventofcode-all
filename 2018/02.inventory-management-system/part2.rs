
use std::io;
use std::io::prelude::*;
use std::collections::*;

fn main() {
    let stdin = io::stdin();
    let mut seen = HashMap::new();
    let res = 'outer: loop {
        for line in stdin.lock().lines() {
            let l = line.unwrap();
            for i in 0..l.len() {
                let s = l[0..i].to_owned() + &l[i + 1..l.len()];
                if seen.contains_key(&s) && seen[&s] != l {
                    break 'outer s;
                }
                seen.insert(s.to_owned(), l.to_owned());
            }
        }
    };
    println!("Result: {}", res);
}
