
use std::io;
use std::io::prelude::*;
use std::collections::*;

const N: usize = 20;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|x| x.unwrap()).collect::<Vec<_>>();
    let mut trans = vec![0; 32];
    let mut state = VecDeque::new();
    for c in lines[0].chars().skip(15) {
        state.push_back(if c == '#' { 1 } else { 0 });
    }
    for l in lines.iter().skip(2) {
        let splt = l.split(" => ").collect::<Vec<_>>();
        let mut v = 0;
        for c in splt[0].chars() {
            v *= 2;
            v += if c == '#' { 1 } else { 0 };
        }
        trans[v] = if splt[1] == "#" { 1 } else { 0 };
    }
    let mut off = 0;
    for _ in 0..N {
        let mut next_state = state.clone();
        for i in 0..state.len() {
            let mut v = 0;
            for j in 0..5 {
                v *= 2;
                if i + j >= 2 && i + j < state.len() + 2 {
                    v += state[i + j - 2];
                }
            }
            next_state[i] = trans[v];
        }
        let s1 = trans[state[0]];
        let s2 = trans[state[0] * 2 + state[1]];
        if s1 + s2 > 0 {
            next_state.push_front(s2);
            off += 1;
            if s1 > 0 {
                next_state.push_front(s1);
                off += 1;
            }
        }
        let e2 = trans[state[state.len() - 2] * 16 + state[state.len() - 1] * 8];
        let e1 = trans[state[state.len() - 1] * 16];
        if e1 + e2 > 0 {
            next_state.push_back(e2);
            if e1 > 0 {
                next_state.push_back(e1);
            }
        }
        state = next_state;
    }
    println!("Result: {}", state.iter().enumerate().map(|e| *e.1 as i64 * (e.0 as i64 - off)).sum::<i64>());
}

