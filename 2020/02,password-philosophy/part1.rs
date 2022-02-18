
use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let mut res = 0;
    for l in stdin.lock().lines() {
        let line = l.unwrap();
        let splt = line.split(": ").collect::<Vec<_>>();
        let rule = splt[0].split(" ").collect::<Vec<_>>();
        let range = rule[0].split("-").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();
        let c = rule[1].chars().next().unwrap();
        let mut count = 0;
        for x in splt[1].chars() {
            if c == x {
                count += 1;
            }
        }
        if count >= range[0] && count <= range[1] {
            res += 1;
        }
    }
    println!("Result: {}", res);
}

