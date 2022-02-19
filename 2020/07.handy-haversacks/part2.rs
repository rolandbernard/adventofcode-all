
use std::io;
use std::io::prelude::*;
use std::collections::*;

struct Graph {
    nodes: HashMap<String, usize>,
    names: Vec<String>,
    conns: Vec<Vec<(usize, usize)>>,
}

impl Graph {
    fn new() -> Graph {
        return Graph { nodes: HashMap::new(), names: Vec::new(), conns: Vec::new() };
    }

    fn get_node(&mut self, name: &str) -> usize {
        if self.nodes.contains_key(name) {
            return self.nodes[name];
        } else {
            let id = self.names.len();
            self.names.push(name.to_owned());
            self.nodes.insert(name.to_owned(), id);
            self.conns.push(Vec::new());
            return id;
        }
    }

    fn connect(&mut self, from: &str, to: &str, weight: usize) {
        let f = self.get_node(from);
        let t = self.get_node(to);
        self.conns[f].push((weight, t));
    }
}

fn required(g: &Graph, f: usize) -> usize {
    let mut sum = 1;
    for &(w, x) in &g.conns[f] {
        sum += w * required(g, x);
    }
    return sum;
}

fn main() {
    let stdin = io::stdin();
    let mut rules = Graph::new();
    for l in stdin.lock().lines() {
        let line = l.unwrap();
        let splt = line.split(" bags contain ").collect::<Vec<_>>();
        if &splt[1][..2] != "no" {
            for c in splt[1].trim_matches('.').split(", ") {
                let cont = c.trim_matches('s');
                let len = cont.len();
                let start = cont.find(' ').unwrap() + 1;
                rules.connect(splt[0], &cont[start..len - 4], cont[..start - 1].parse::<usize>().unwrap());
            }
        }
    }
    let gold = rules.get_node("shiny gold");
    println!("Result: {}", required(&rules, gold) - 1);
}

