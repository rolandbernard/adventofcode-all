
use std::io;
use std::io::prelude::*;
use std::collections::*;

struct Graph {
    nodes: HashMap<String, usize>,
    names: Vec<String>,
    conns: Vec<Vec<usize>>,
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

    fn connect(&mut self, from: &str, to: &str) {
        let f = self.get_node(from);
        let t = self.get_node(to);
        self.conns[f].push(t);
    }

    fn reachable(&self, from: usize) -> usize {
        fn color_reachable(g: &Graph, at: usize, seen: &mut Vec<bool>) {
            if !seen[at] {
                seen[at] = true;
                for &next in &g.conns[at] {
                    color_reachable(g, next, seen);
                }
            }
        }
        let mut seen = vec![false; self.nodes.len()];
        color_reachable(self, from, &mut seen);
        return seen.iter().filter(|&&x| x).count();
    }
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
                rules.connect(&cont[start..len - 4], splt[0]);
            }
        }
    }
    let gold = rules.get_node("shiny gold");
    println!("Result: {}", rules.reachable(gold) - 1);
}

