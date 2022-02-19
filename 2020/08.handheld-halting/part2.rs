
use std::io;
use std::io::prelude::*;

#[derive(Copy, Clone)]
enum Instr {
    Nop(i64),
    Acc(i64),
    Jmp(i64),
}

#[derive(Copy, Clone)]
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

fn find_termination(mut cpu: CpuState, insts: &Vec<Instr>, seen: &mut Vec<bool>, can: bool) -> Option<i64> {
    if cpu.pc as usize == insts.len() {
        return Some(cpu.acc);
    } else if cpu.pc < 0 || seen[cpu.pc as usize] {
        return None;
    } else {
        let loc = cpu.pc as usize;
        seen[loc] = true;
        if can {
            if let Instr::Nop(x) = insts[loc] {
                let mut cpu2 = cpu;
                run_instr(&mut cpu2, Instr::Jmp(x));
                if let Some(x) = find_termination(cpu2, insts, seen, false) {
                    seen[loc] = false;
                    return Some(x);
                }
            } else if let Instr::Jmp(x) = insts[loc] {
                let mut cpu2 = cpu;
                run_instr(&mut cpu2, Instr::Nop(x));
                if let Some(x) = find_termination(cpu2, insts, seen, false) {
                    seen[loc] = false;
                    return Some(x);
                }
            }
        }
        run_instr(&mut cpu, insts[loc]);
        let res = find_termination(cpu, insts, seen, can);
        seen[loc] = false;
        return res;
    }
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
    println!("Result: {}", find_termination(CpuState { acc: 0, pc: 0 }, &insts, &mut seen, true).unwrap());
}

