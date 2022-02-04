
use std::io;
use std::io::prelude::*;
use std::collections::*;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|x| x.unwrap()).collect::<Vec<String>>();
    lines.sort();
    let mut guard_sleep = HashMap::new();
    let mut guard = 0;
    let mut start = 0;
    for line in &lines {
        let splt = line.split(" ").collect::<Vec<&str>>();
        let time = splt[1][3..5].parse::<usize>().unwrap();
        match splt[2] {
            "Guard" => {
                guard = splt[3][1..splt[3].len()].parse::<usize>().unwrap();
                if !guard_sleep.contains_key(&guard) {
                    guard_sleep.insert(guard, vec![0;60]);
                }
            },
            "falls" => {
                start = time;
            },
            "wakes" => {
                for i in start..time {
                    guard_sleep.get_mut(&guard).unwrap()[i] += 1;
                }
            },
            _ => {}
        }
    }
    let mut best = 0;
    for (g, map) in guard_sleep {
        for i in 0..map.len() {
            if map[i] > best {
                best = map[i];
                guard = g;
                start = i;
            }
        }
    }
    println!("Result: {}", guard * start);
}
