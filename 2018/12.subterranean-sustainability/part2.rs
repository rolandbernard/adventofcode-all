
use std::io;
use std::io::prelude::*;
use std::collections::*;

const N: i64 = 50000000000;

fn shape(set: &HashSet<i64>) -> HashSet<i64> {
    let min = set.iter().min().unwrap();
    return set.iter().filter(|&x| x != min).map(|x| x - min).collect();
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|x| x.unwrap()).collect::<Vec<_>>();
    let mut trans = vec![false; 32];
    let mut state = HashSet::new();
    for (i, c) in lines[0].chars().skip(15).enumerate() {
        if c == '#' {
            state.insert(i as i64);
        }
    }
    for l in lines.iter().skip(2) {
        let splt = l.split(" => ").collect::<Vec<_>>();
        let mut v = 0;
        for c in splt[0].chars() {
            v *= 2;
            v += if c == '#' { 1 } else { 0 };
        }
        trans[v] = if splt[1] == "#" { true } else { false };
    }
    for g in 0..N {
        let mut to_change = HashSet::new();
        for &i in &state {
            for j in 0..5 {
                to_change.insert(i + j - 2);
            }
        }
        let mut new_state = HashSet::new();
        for i in to_change {
            let mut v = 0;
            for j in 0..5 {
                v *= 2;
                v += if state.contains(&(i + j - 2)) { 1 } else { 0 };
            }
            if trans[v] {
                new_state.insert(i);
            }
        }
        if shape(&state) == shape(&new_state) {
            let delta = new_state.iter().min().unwrap() - state.iter().min().unwrap();
            state = state.iter().map(|x| x + delta * (N - g)).collect();
            break;
        } else {
            state = new_state;
        }
    }
    println!("Result: {}", state.iter().sum::<i64>());
}

