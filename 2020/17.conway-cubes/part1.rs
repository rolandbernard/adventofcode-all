
use std::io;
use std::io::prelude::*;
use std::collections::*;

fn next_map(map: HashSet<(i64, i64, i64)>) -> HashSet<(i64, i64, i64)> {
    let mut to_inspect = HashSet::new();
    for &(i, j, k) in &map {
        for di in -1..=1 {
            for dj in -1..=1 {
                for dk in -1..=1 {
                    to_inspect.insert((i + di, j + dj, k + dk));
                }
            }
        }
    }
    let mut new = HashSet::new();
    for (i, j, k) in to_inspect {
        let mut occ = 0;
        for di in -1..=1 as i64 {
            for dj in -1..=1 as i64 {
                for dk in -1..=1 as i64 {
                    if (di != 0 || dj != 0 || dk != 0) && map.contains(&(i + di, j + dj, k + dk)) {
                        occ += 1;
                    }
                }
            }
        }
        if (map.contains(&(i, j, k)) && occ == 2) || occ == 3 {
            new.insert((i, j, k));
        }
    }
    return new;
}

fn main() {
    let stdin = io::stdin();
    let mut map = HashSet::new();
    for (i, l) in stdin.lock().lines().enumerate() {
        let lines = l.unwrap();
        for (j, c) in lines.chars().enumerate() {
            if c == '#' {
                map.insert((i as i64, j as i64, 0));
            }
        }
    }
    for _ in 0..6 {
        map = next_map(map);
    }
    println!("Result: {}", map.len());
}

