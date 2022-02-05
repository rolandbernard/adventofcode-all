
use std::io;
use std::io::prelude::*;
use std::collections::*;

fn main() {
    let stdin = io::stdin();
    let points = stdin.lock().lines()
        .map(|x| {
            let splt = x.unwrap().split(", ")
                .map(|y| y.parse().unwrap())
                .collect::<Vec<usize>>();
            (splt[0], splt[1])
        })
        .collect::<Vec<(usize, usize)>>();
    let mut lower = (usize::MAX, usize::MAX);
    let mut upper = (0, 0);
    for p in &points {
        upper.0 = upper.0.max(p.0);
        upper.1 = upper.1.max(p.1);
        lower.0 = lower.0.min(p.0);
        lower.1 = lower.1.min(p.1);
    }
    let mut map = vec![vec![(usize::MAX, usize::MAX); upper.1 - lower.1 + 1]; upper.0 - lower.0 + 1];
    let mut queue = VecDeque::new();
    for (i, p) in points.iter().enumerate() {
        queue.push_back((p.0, p.1, i, 1));
    }
    let mut sizes = vec![0; points.len()];
    let mut at_edge = vec![false; points.len()];
    while !queue.is_empty() {
        let (x, y, i, d) = queue.pop_front().unwrap();
        if x >= lower.0 && x <= upper.0 && y >= lower.1 && y <= upper.1 {
            let field = &mut map[x - lower.0][y - lower.1];
            if field.1 >= d && field.0 != i {
                if field.0 != usize::MAX {
                    sizes[field.0] -= 1;
                }
                if field.1 > d {
                    sizes[i] += 1;
                    *field = (i, d);
                    if x == lower.0 || x == upper.0 || y == lower.1 || x == upper.1 {
                        at_edge[i] = true;
                    }
                } else {
                    *field = (usize::MAX, 0);
                }
                queue.push_back((x + 1, y, i, d + 1));
                queue.push_back((x - 1, y, i, d + 1));
                queue.push_back((x, y + 1, i, d + 1));
                queue.push_back((x, y - 1, i, d + 1));
            }
        }
    }
    let mut best = 0;
    for i in 0..points.len() {
        if !at_edge[i] && sizes[i] > best {
            best = sizes[i];
        }
    }
    println!("Result: {}", best);
}

