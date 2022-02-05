
use std::io;
use std::io::prelude::*;
use std::collections::*;

fn dist_to_all(pos: (i64, i64), points: &Vec<(i64, i64)>) -> i64 {
    let mut sum = 0;
    for p in points {
        sum += (p.0 - pos.0).abs() + (p.1 - pos.1).abs();
    }
    return sum;
}

fn main() {
    let stdin = io::stdin();
    let points = stdin.lock().lines()
        .map(|x| {
            let splt = x.unwrap().split(", ")
                .map(|y| y.parse().unwrap())
                .collect::<Vec<i64>>();
            (splt[0], splt[1])
        })
        .collect::<Vec<(i64, i64)>>();
    let mut center = (0, 0);
    for p in &points {
        center.0 += p.0;
        center.1 += p.1;
    }
    center.0 /= points.len() as i64;
    center.1 /= points.len() as i64;
    let mut queue = VecDeque::new();
    queue.push_back(center);
    let mut visited = HashSet::new();
    while !queue.is_empty() {
        let pos = queue.pop_front().unwrap();
        if !visited.contains(&pos) && dist_to_all(pos, &points) < 10000 {
            visited.insert(pos);
            queue.push_back((pos.0 + 1, pos.1));
            queue.push_back((pos.0 - 1, pos.1));
            queue.push_back((pos.0, pos.1 + 1));
            queue.push_back((pos.0, pos.1 - 1));
        }
    }
    println!("Result: {}", visited.len());
}

