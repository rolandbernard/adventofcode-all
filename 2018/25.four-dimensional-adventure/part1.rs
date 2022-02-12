
use std::io;
use std::io::prelude::*;

fn dist(a: &Vec<i64>, b: &Vec<i64>) -> i64 {
    return a.iter().enumerate().map(|(i, x)| (x - b[i]).abs()).sum();
}

fn mark(i: usize, graph: &mut Vec<(bool, Vec<usize>)>) {
    if graph[i].0 {
        graph[i].0 = false;
        for j in 0..graph[i].1.len() {
            mark(graph[i].1[j], graph);
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut points = Vec::new();
    for l in stdin.lock().lines() {
        let line = l.unwrap();
        let splt = line.split(",").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();
        points.push(splt);
    }
    let mut graph = Vec::new();
    for i in 0..points.len() {
        graph.push((true, Vec::new()));
        for j in 0..points.len() {
            if dist(&points[i], &points[j]) <= 3 {
                graph[i].1.push(j);
            }
        }
    }
    let mut count = 0;
    for i in 0..points.len() {
        if graph[i].0 {
            count += 1;
            mark(i, &mut graph);
        }
    }
    println!("Result: {}", count);
}

