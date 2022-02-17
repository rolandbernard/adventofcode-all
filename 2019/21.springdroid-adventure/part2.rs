
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
"OR E J
AND I J
OR H J
AND D J
OR C T
AND B T
NOT T T
AND T J
NOT A T
OR T J
RUN
";

fn main() {
    let stdin = io::stdin();
    let mem = intcode::parse(&stdin.lock().lines().next().unwrap().unwrap());
    println!("Result: {}", execute(mem, PROGRAM));
}

