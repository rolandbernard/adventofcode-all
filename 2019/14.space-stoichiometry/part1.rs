
use std::io;
use std::io::prelude::*;
use std::collections::*;

fn create(kind: &str, amm: usize, left: &mut HashMap<String, usize>, rec: &HashMap<String, (usize, Vec<(usize, String)>)>) -> usize {
    if kind == "ORE" {
        return amm;
    } else {
        let mut rest = 0;
        if left.contains_key(kind) {
            rest = left[kind];
        }
        let mut req = 0;
        if amm > rest {
            let count = ((amm - rest) + rec[kind].0 - 1) / rec[kind].0;
            for g in &rec[kind].1 {
                req += create(&g.1, g.0 * count, left, rec);
            }
            rest += count * rec[kind].0;
        }
        rest -= amm;
        left.insert(kind.to_owned(), rest);
        return req;
    }
}

fn main() {
    let stdin = io::stdin();
    let mut rec = HashMap::new();
    for l in stdin.lock().lines() {
        let line = l.unwrap();
        let splt = line.split(" => ")
            .map(|x| x.split(", ").map(|x| {
                let parts = x.split(" ").collect::<Vec<_>>();
                return (parts[0].parse::<usize>().unwrap(), parts[1].to_owned());
            }).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        rec.insert(splt[1][0].1.to_owned(), (splt[1][0].0, splt[0].clone()));
    }
    let mut left = HashMap::new();
    println!("Result: {}", create("FUEL", 1, &mut left, &rec));
}

