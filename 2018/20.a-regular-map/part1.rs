
use std::io;
use std::io::prelude::*;
use std::collections::*;

fn moving(pos: (i64, i64, i64), dir: char) -> (i64, i64, i64) {
    match dir {
        'N' => (pos.0, pos.1 - 1, pos.2 + 1),
        'S' => (pos.0, pos.1 + 1, pos.2 + 1),
        'E' => (pos.0 - 1, pos.1, pos.2 + 1),
        'W' => (pos.0 + 1, pos.1, pos.2 + 1),
        _ => pos,
    }
}

fn main() {
    let stdin = io::stdin();
    let path = stdin.lock().lines().next().unwrap().unwrap();
    let mut stack = Vec::new();
    let mut heads = Vec::new();
    let mut dists = HashMap::new();
    heads.push((0, 0, 0));
    for c in path.chars() {
        match c {
            '(' => {
                stack.push((heads.clone(), Vec::new()));
            },
            '|' => {
                let stack_top = stack.len() - 1;
                stack[stack_top].1.push(heads);
                heads = stack[stack_top].0.clone();
            },
            ')' => {
                let stack_top = stack.len() - 1;
                stack[stack_top].1.push(heads);
                heads = stack.pop().unwrap().1.into_iter().flatten().collect();
            },
            'N' | 'S' | 'E' | 'W' => {
                for h in &mut heads {
                    *h = moving(*h, c);
                }
                let mut j = 0;
                for i in 0..heads.len() {
                    let h = heads[i];
                    heads[j] = h;
                    let pos = (h.0, h.1);
                    if !dists.contains_key(&pos) || dists[&pos] > h.2 {
                        dists.insert(pos, h.2);
                        j += 1;
                    }
                }
                heads.resize(j, (0, 0, 0));
            },
            _ => {},
        }
    }
    println!("Result: {}", dists.values().max().unwrap());
}

