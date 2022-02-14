
use std::io;
use std::io::prelude::*;

#[path ="../intcode/intcode.rs"]
mod intcode;

fn main() {
    let stdin = io::stdin();
    let mut mem = intcode::parse(&stdin.lock().lines().next().unwrap().unwrap());
    let mut cpu = intcode::IntState::new();
    let mut vals = Vec::new();
    while !cpu.halt {
        intcode::run_instr(&mut mem, &mut cpu, || 0, |x| { vals.push(x); });
    }
    let mut count = 0;
    for i in (0..vals.len()).step_by(3) {
        if vals[i + 2] == 2 {
            count += 1;
        }
    }
    println!("Result: {}", count);
}

