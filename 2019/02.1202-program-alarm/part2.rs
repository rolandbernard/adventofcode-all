
use std::io;
use std::io::prelude::*;

#[path ="../intcode/intcode.rs"]
mod intcode;

fn run_prog(mut mem: intcode::IntMem, x: (i64, i64)) -> bool {
    mem[1] = x.0;
    mem[2] = x.1;
    let mut cpu = intcode::IntState::new();
    while !cpu.halt {
        intcode::run_instr(&mut mem, &mut cpu, || 0, |_| {});
    }
    return mem[0] == 19690720;
}

fn main() {
    let stdin = io::stdin();
    let mem = intcode::parse(&stdin.lock().lines().next().unwrap().unwrap());
    let res = 'outer: loop {
        for i in 0..100 {
            for j in 0..100 {
                if run_prog(mem.clone(), (i, j)) {
                    break 'outer 100 * i + j;
                }
            }
        }
    };
    println!("Result: {}", res);
}

