
use std::io;
use std::io::prelude::*;
use std::collections::*;

fn moves(o: &str, graph: &HashMap<String, Vec<String>>) -> Option<(i64, i64)> {
    if graph.contains_key(o) {
        let childs = graph[o].iter().map(|x| moves(x, graph))
            .filter(|x| x.is_some()).map(|x| x.unwrap()).collect::<Vec<_>>();
        return match childs.len() {
            1 => Some((childs[0].0 + 1, childs[0].1)),
            2 => Some((0, childs[0].0 + childs[1].0)),
            _ => None,
        }
    } else if o == "SAN" || o == "YOU" {
        return Some((0, 0));
    } else {
        return None;
    }
}

fn main() {
    let stdin = io::stdin();
    let mut graph = HashMap::new();
    for l in stdin.lock().lines() {
        let line = l.unwrap();
        let splt = line.split(")").collect::<Vec<_>>();
        graph.entry(splt[0].to_owned()).or_insert(Vec::new()).push(splt[1].to_owned());
    }
    println!("Result: {}", moves("COM", &graph).unwrap().1);
}

