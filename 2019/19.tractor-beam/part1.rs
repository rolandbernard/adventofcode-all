
use std::io;
use std::io::prelude::*;

#[path ="../intcode/intcode.rs"]
mod intcode;

fn value_at(mut mem: intcode::IntMem, pos: (i64, i64)) -> i64 {
    let mut out = None;
    let inp_arr = [pos.0, pos.1];
    let mut inp = inp_arr.iter();
    let mut cpu = intcode::IntState::new();
    while out == None {
        intcode::run_instr(&mut mem, &mut cpu, || *inp.next().unwrap(), |x| out = Some(x));
    }
    return out.unwrap();
}

fn main() {
    let stdin = io::stdin();
    let mem = intcode::parse(&stdin.lock().lines().next().unwrap().unwrap());
    let mut count = 0;
    for i in 0..50 {
        for j in 0..50 {
            count += value_at(mem.clone(), (i, j));
        }
    }
    println!("Result: {}", count);
}

