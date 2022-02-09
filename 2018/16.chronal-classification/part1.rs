
use std::io;
use std::io::prelude::*;

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
    let mut i = 0;
    let mut count = 0;
    while lines[i] != "" {
        let before = lines[i][9..lines[i].len() - 1].split(", ")
            .map(|x| x.parse().unwrap()).collect::<Vec<i64>>();
        let instr = lines[i + 1].split(" ")
            .map(|x| x.parse().unwrap()).collect::<Vec<i64>>();
        let after = lines[i + 2][9..lines[i].len() - 1].split(", ")
            .map(|x| x.parse().unwrap()).collect::<Vec<i64>>();
        let mut pos = 0;
        for i in 0..16 {
            let mut state = before.clone();
            run_instr(&mut state, i, instr[1], instr[2], instr[3]);
            if state == after {
                pos += 1;
            }
        }
        if pos >= 3{
            count += 1;
        }
        i += 4;
    }
    println!("Result: {}", count);
}

