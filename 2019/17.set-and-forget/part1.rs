
use std::io;
use std::io::prelude::*;

#[path ="../intcode/intcode.rs"]
mod intcode;

fn main() {
    let stdin = io::stdin();
    let mut mem = intcode::parse(&stdin.lock().lines().next().unwrap().unwrap());
    let mut cpu = intcode::IntState::new();
    let mut out = Vec::new();
    out.push(Vec::new());
    while !cpu.halt {
        intcode::run_instr(&mut mem, &mut cpu, || 0, |x| {
            if x == 10 {
                out.push(Vec::new());
            } else {
                let last = out.len() - 1;
                out[last].push(match x as u8 as char {
                    '#' => 1,
                    '^' => 2,
                    'v' => 3,
                    '>' => 4,
                    '<' => 5,
                    _ => 0,
                });
            }
        });
    }
    while out.last().unwrap().len() == 0 {
        out.pop();
    }
    let mut sum = 0;
    for i in 1..out.len() - 1 {
        for j in 1..out[i].len() - 1 {
            if out[i - 1][j] == 1 && out[i + 1][j] == 1 && out[i][j - 1] == 1 && out[i][j + 1] == 1 {
                sum += i * j;
            }
        }
    }
    println!("Result: {}", sum);
}

