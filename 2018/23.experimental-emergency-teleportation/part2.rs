
use std::io;
use std::io::prelude::*;
use std::collections::*;

fn solve_rec(
    r: HashSet<usize>, mut p: HashSet<usize>, mut x: HashSet<usize>,
    graph: &Vec<HashSet<usize>>, nodes: &Vec<((i64, i64, i64), i64)>, best: &mut (usize, i64)
) {
    if p.is_empty() && x.is_empty() {
        if r.len() >= best.0 {
            let dist = r.iter().map(|x| nodes[*x]).map(|x| x.0.0.abs() + x.0.1.abs() + x.0.2.abs() - x.1).max().unwrap();
            if r.len() > best.0 || dist < best.1 {
                *best = (r.len(), dist);
            }
        }
    } else {
        let u = p.iter().chain(x.iter()).max_by_key(|x| graph[**x].len()).unwrap();
        for v in &p - &graph[*u] {
            solve_rec(&r | &HashSet::from([v]), &p & &graph[v], &x & &graph[v], graph, nodes, best);
            p.remove(&v);
            x.insert(v);
        }
    }
}

fn solve(graph: &Vec<HashSet<usize>>, nodes: &Vec<((i64, i64, i64), i64)>) -> i64 {
    let mut best = (0, 0);
    let mut p = (0..graph.len()).collect::<HashSet<_>>();
    let mut x = HashSet::new();
    let mut all = graph.iter().enumerate().collect::<Vec<_>>();
    all.sort_by_key(|x| x.1.len());
    for (v, nei) in all {
        solve_rec(HashSet::from([v]), nei & &p, nei & &x, graph, nodes, &mut best);
        p.remove(&v);
        x.insert(v);
    }
    return best.1;
}

fn main() {
    let stdin = io::stdin();
    let mut bots = Vec::new();
    for l in stdin.lock().lines() {
        let line = l.unwrap();
        let splt = line.split(|c| c == '<' || c == '>' || c == '=' || c == ',')
            .map(|x| x.parse::<i64>().ok()).collect::<Vec<Option<i64>>>();
        bots.push(((splt[2].unwrap(), splt[3].unwrap(), splt[4].unwrap()), (splt[7].unwrap())));
    }
    let mut graph = vec![HashSet::new(); bots.len()];
    for i in 0..bots.len() {
        for j in 0..bots.len() {
            let b = bots[i];
            let m = bots[j];
            if i != j && (b.0.0 - m.0.0).abs() + (b.0.1 - m.0.1).abs() + (b.0.2 - m.0.2).abs() <= b.1 + m.1 {
                graph[i].insert(j);
            }
        }
    }
    println!("Result: {}", solve(&graph, &bots));
}

