
use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let mut res = 0;
    for l in stdin.lock().lines() {
        let line = l.unwrap();
        let splt = line.split(": ").collect::<Vec<_>>();
        let rule = splt[0].split(" ").collect::<Vec<_>>();
        let range = rule[0].split("-").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
        let c = rule[1].chars().next().unwrap();
        let count = splt[1].chars().enumerate()
            .filter(|&(i, x)| x == c && (i + 1 == range[0] || i + 1 == range[1])).count();
        if count == 1 {
            res += 1;
        }
    }
    println!("Result: {}", res);
}

