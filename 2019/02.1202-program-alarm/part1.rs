
use std::io;
use std::io::prelude::*;

#[path ="../intcode/intcode.rs"]
mod intcode;

fn main() {
    let stdin = io::stdin();
    let mut mem = intcode::parse(&stdin.lock().lines().next().unwrap().unwrap());
    let mut cpu = intcode::IntState::new();
    mem[1] = 12;
    mem[2] = 2;
    while !cpu.halt {
        intcode::run_instr(&mut mem, &mut cpu, || 0, |_| {});
    }
    println!("Result: {}", mem[0]);
}

