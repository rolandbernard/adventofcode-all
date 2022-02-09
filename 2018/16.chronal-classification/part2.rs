
use std::io;
use std::io::prelude::*;
use std::collections::*;

fn run_instr(state: &mut Vec<i64>, op: i64, a: i64, b: i64, c: i64) {
    match op {
        0 => state[c as usize] = state[a as usize] + state[b as usize],
        1 => state[c as usize] = state[a as usize] + b,
        2 => state[c as usize] = state[a as usize] * state[b as usize],
        3 => state[c as usize] = state[a as usize] * b,
        4 => state[c as usize] = state[a as usize] & state[b as usize],
        5 => state[c as usize] = state[a as usize] & b,
        6 => state[c as usize] = state[a as usize] | state[b as usize],
        7 => state[c as usize] = state[a as usize] | b,
        8 => state[c as usize] = state[a as usize],
        9 => state[c as usize] = a,
        10 => state[c as usize] = if a > state[b as usize] { 1 } else { 0 },
        11 => state[c as usize] = if state[a as usize] > b { 1 } else { 0 },
        12 => state[c as usize] = if state[a as usize] > state[b as usize] { 1 } else { 0 },
        13 => state[c as usize] = if a == state[b as usize] { 1 } else { 0 },
        14 => state[c as usize] = if state[a as usize] == b { 1 } else { 0 },
        15 => state[c as usize] = if state[a as usize] == state[b as usize] { 1 } else { 0 },
        _ => {},
    }
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines()
        .map(|x| x.unwrap())
        .collect::<Vec<_>>();
    let mut pos = vec![(0..16).collect::<HashSet<_>>(); 16];
    let mut i = 0;
    while lines[i] != "" {
        let before = lines[i][9..lines[i].len() - 1].split(", ")
            .map(|x| x.parse().unwrap()).collect::<Vec<i64>>();
        let instr = lines[i + 1].split(" ")
            .map(|x| x.parse().unwrap()).collect::<Vec<i64>>();
        let after = lines[i + 2][9..lines[i].len() - 1].split(", ")
            .map(|x| x.parse().unwrap()).collect::<Vec<i64>>();
        for i in 0..16 {
            let mut state = before.clone();
            run_instr(&mut state, i, instr[1], instr[2], instr[3]);
            if state != after {
                pos[instr[0] as usize].remove(&i);
            }
        }
        i += 4;
    }
    let mut trans = vec![0; 16];
    for _ in 0..16 {
        let idx = 'outer: loop {
            for i in 0..16 {
                if pos[i].len() == 1 {
                    break 'outer i;
                }
            }
        };
        trans[idx] = *pos[idx].iter().next().unwrap();
        for i in 0..16 {
            pos[i].remove(&trans[idx]);
        }
    }
    i += 2;
    let mut state = vec![0; 4];
    while i < lines.len() {
        let instr = lines[i].split(" ")
            .map(|x| x.parse().unwrap()).collect::<Vec<i64>>();
        run_instr(&mut state, trans[instr[0] as usize], instr[1], instr[2], instr[3]);
        i += 1;
    }
    println!("Result: {}", state[0]);
}

