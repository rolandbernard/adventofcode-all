
use std::io;
use std::io::prelude::*;

enum Instr {
    Nop,
    SetI { dst: usize, val: i64 },
    SetR { dst: usize, src: usize },
    SubI { dst: usize, val: i64 },
    SubR { dst: usize, src: usize },
    MulI { dst: usize, val: i64 },
    MulR { dst: usize, src: usize },
    JnzRI { cnd: usize, off: i64 },
    JnzRR { cnd: usize, off: usize },
    JnzII { cnd: i64, off: i64 },
    JnzIR { cnd: i64, off: usize },
}

struct CpuState {
    pc: usize,
    regs: [i64; 8],
}

fn read_program() -> Vec<Instr> {
    let stdin = io::stdin();
    let mut prog = Vec::new();
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        let x = l.split(" ").collect::<Vec<&str>>();
        let instr;
        match x[0] {
            "set" | "sub" | "mul" => {
                let dst = x[1].bytes().next().unwrap() as usize - 'a' as usize;
                if x[2].chars().next().unwrap().is_ascii_lowercase() {
                    let src = x[2].bytes().next().unwrap() as usize - 'a' as usize;
                    instr = match x[0] {
                        "set" => Instr::SetR { dst, src },
                        "sub" => Instr::SubR { dst, src },
                        "mul" => Instr::MulR { dst, src },
                        _ => Instr::Nop,
                    };
                } else {
                    let val = x[2].parse().unwrap();
                    instr = match x[0] {
                        "set" => Instr::SetI { dst, val },
                        "sub" => Instr::SubI { dst, val },
                        "mul" => Instr::MulI { dst, val },
                        _ => Instr::Nop,
                    };
                }
            },
            "jnz" => {
                if x[1].chars().next().unwrap().is_ascii_lowercase() {
                    let cnd = x[1].bytes().next().unwrap() as usize - 'a' as usize;
                    if x[2].chars().next().unwrap().is_ascii_lowercase() {
                        let off = x[2].bytes().next().unwrap() as usize - 'a' as usize;
                        instr = Instr::JnzRR { cnd, off };
                    } else {
                        let off = x[2].parse().unwrap();
                        instr = Instr::JnzRI { cnd, off };
                    }
                } else {
                    let cnd = x[1].parse().unwrap();
                    if x[2].chars().next().unwrap().is_ascii_lowercase() {
                        let off = x[2].bytes().next().unwrap() as usize - 'a' as usize;
                        instr = Instr::JnzIR { cnd, off };
                    } else {
                        let off = x[2].parse().unwrap();
                        instr = Instr::JnzII { cnd, off };
                    }
                }
            }
            _ => {
                println!("Error: '{}'", l);
                instr = Instr::Nop;
            }
        }
        prog.push(instr);
    }
    return prog;
}

fn run_program(prog: &Vec<Instr>, cpu: &mut CpuState) {
    while cpu.pc < 8 {
        match prog[cpu.pc] {
            Instr::Nop => {},
            Instr::SetI { dst, val } => { cpu.regs[dst] = val },
            Instr::SetR { dst, src } => { cpu.regs[dst] = cpu.regs[src] },
            Instr::SubI { dst, val } => { cpu.regs[dst] -= val },
            Instr::SubR { dst, src } => { cpu.regs[dst] -= cpu.regs[src] },
            Instr::MulI { dst, val } => { cpu.regs[dst] *= val },
            Instr::MulR { dst, src } => { cpu.regs[dst] *= cpu.regs[src] },
            Instr::JnzRI { cnd, off } => if cpu.regs[cnd] != 0 { cpu.pc = cpu.pc.wrapping_add((off - 1) as usize) },
            Instr::JnzRR { cnd, off } => if cpu.regs[cnd] != 0 { cpu.pc = cpu.pc.wrapping_add(cpu.regs[off] as usize) },
            Instr::JnzII { cnd, off } => if cnd != 0 { cpu.pc = cpu.pc.wrapping_add((off - 1) as usize) },
            Instr::JnzIR { cnd, off } => if cnd != 0 { cpu.pc = cpu.pc.wrapping_add(cpu.regs[off] as usize) },
        }
        cpu.pc += 1;
    }
}

fn is_prime(n: i64) -> bool {
    if n < 2 {
        return false;
    } else if n % 2 == 0 {
        return n == 2;
    } else if n % 3 == 0 {
        return n == 3;
    } else {
        for i in (5..).take_while(|x| x*x <= n) {
            if n % i == 0 {
                return false;
            }
        }
        return true;
    }
}

fn count_non_primes(from: i64, to: i64) -> i64 {
    let mut count = 0;
    for i in (from..=to).step_by(17) {
        if !is_prime(i) {
            count += 1;
        }
    }
    return count;
}

fn main() {
    let prog = read_program();
    let mut cpu = CpuState { pc: 0, regs: [1, 0, 0, 0, 0, 0, 0, 0] };
    run_program(&prog, &mut cpu);
    println!("Result: {}", count_non_primes(cpu.regs[1], cpu.regs[2]));
}
