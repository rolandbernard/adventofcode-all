
use std::io;
use std::io::prelude::*;
use std::collections::*;

type Rec = HashMap<String, (usize, Vec<(usize, String)>)>;

fn create(kind: &str, amm: usize, left: &mut HashMap<String, usize>, rec: &Rec) -> usize {
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

fn create_fuel(amm: usize, rec: &Rec) -> usize {
    let mut left = HashMap::new();
    return create("FUEL", amm, &mut left, rec);
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
    let mut i = 0;
    let mut j = 1000000000000;
    while i + 1 < j {
        let m = (i + j) / 2;
        let ore = create_fuel(m, &rec);
        if ore > 1000000000000 {
            j = m;
        } else {
            i = m;
        }
    }
    println!("Result: {}", i);
}

