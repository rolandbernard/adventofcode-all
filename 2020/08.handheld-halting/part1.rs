
use std::io;
use std::io::prelude::*;

#[derive(Copy, Clone)]
enum Instr {
    Nop(i64),
    Acc(i64),
    Jmp(i64),
}

struct CpuState {
    acc: i64,
    pc: i64,
}

fn run_instr(cpu: &mut CpuState, instr: Instr) {
    match instr {
        Instr::Nop(_) => {},
        Instr::Acc(x) => cpu.acc += x,
        Instr::Jmp(x) => cpu.pc += x - 1,
    }
    cpu.pc += 1;
}

fn main() {
    let stdin = io::stdin();
    let mut insts = Vec::new();
    for l in stdin.lock().lines() {
        let line = l.unwrap();
        let splt = line.split(" ").collect::<Vec<_>>();
        let num = splt[1].parse::<i64>().unwrap();
        if splt[0] == "nop" {
            insts.push(Instr::Nop(num));
        } else if splt[0] == "acc" {
            insts.push(Instr::Acc(num));
        } else if splt[0] == "jmp" {
            insts.push(Instr::Jmp(num));
        } else {
            panic!();
        }
    }
    let mut seen = vec![false; insts.len()];
    let mut cpu = CpuState { acc: 0, pc: 0 };
    while cpu.pc >= 0 && (cpu.pc as usize) < insts.len() && !seen[cpu.pc as usize] {
        let loc = cpu.pc as usize;
        seen[loc] = true;
        run_instr(&mut cpu, insts[loc]);
    }
    println!("Result: {}", cpu.acc);
}

