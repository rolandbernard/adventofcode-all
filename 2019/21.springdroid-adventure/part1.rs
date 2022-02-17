
use std::io;
use std::io::prelude::*;

#[path ="../intcode/intcode.rs"]
mod intcode;

fn execute(mut mem: intcode::IntMem, input: &str) -> i64 {
    let mut out = None;
    let mut inp = input.chars();
    let mut cpu = intcode::IntState::new();
    while !cpu.halt && out == None {
        intcode::run_instr(&mut mem, &mut cpu, || inp.next().unwrap() as u8 as i64, |x| if x > 128 { out = Some(x) });
    }
    return out.unwrap();
}

const PROGRAM: &str =
"NOT A J
NOT B T
OR T J
NOT C T
OR T J
AND D J
WALK
";

fn main() {
    let stdin = io::stdin();
    let mem = intcode::parse(&stdin.lock().lines().next().unwrap().unwrap());
    println!("Result: {}", execute(mem, PROGRAM));
}

