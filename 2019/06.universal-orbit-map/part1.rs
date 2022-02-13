
use std::io;
use std::io::prelude::*;
use std::collections::*;

fn orbits(o: &str, graph: &HashMap<String, Vec<String>>, l: i64) -> i64 {
    let mut sum = l;
    if graph.contains_key(o) {
        for x in &graph[o] {
            sum += orbits(x, graph, l + 1);
        }
    }
    return sum;
}

fn main() {
    let stdin = io::stdin();
    let mut graph = HashMap::new();
    for l in stdin.lock().lines() {
        let line = l.unwrap();
        let splt = line.split(")").collect::<Vec<_>>();
        graph.entry(splt[0].to_owned()).or_insert(Vec::new()).push(splt[1].to_owned());
    }
    println!("Result: {}", orbits("COM", &graph, 0));
}

