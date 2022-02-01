
use std::io;
use std::io::prelude::*;
use std::collections::*;

fn longest_path(
    cur: u64, visited: u64, nodes: &Vec<(u64, u64)>, conn: &HashMap<u64, Vec<(u64, u64)>>,
    cache: &mut HashMap<(u64, u64), (u64, u64)>
) -> (u64, u64) {
    if !cache.contains_key(&(cur, visited)) {
        let mut max = (0, 0);
        for next in &conn[&cur] {
            if (visited & (1 << next.1)) == 0 {
                let fol = longest_path(next.0, visited | (1 << next.1), nodes, conn, cache);
                let dist = (1 + fol.0, cur + next.0 + fol.1);
                if dist > max {
                    max = dist;
                }
            }
        }
        cache.insert((cur, visited), max);
    }
    return cache[&(cur, visited)];
}

fn main() {
    let stdin = io::stdin();
    let mut comps = Vec::new();
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        let s = l.split("/").collect::<Vec<&str>>();
        comps.push((s[0].parse::<u64>().unwrap(), s[1].parse::<u64>().unwrap()));
    }
    let mut next = HashMap::new();
    for i in 0..comps.len() {
        (*next.entry(comps[i].0).or_insert(Vec::new())).push((comps[i].1, i as u64));
        (*next.entry(comps[i].1).or_insert(Vec::new())).push((comps[i].0, i as u64));
    }
    let mut cache = HashMap::new();
    println!("Result: {}", longest_path(0, 0, &comps, &next, &mut cache).1);
}
