
use std::io;
use std::io::prelude::*;

type CpuState = [i64; 6];

#[derive(Copy, Clone)]
enum Instr {
    AddR(usize, usize, usize),
    AddI(usize, i64, usize),
    MulR(usize, usize, usize),
    MulI(usize, i64, usize),
    AndR(usize, usize, usize),
    AndI(usize, i64, usize),
    OrR(usize, usize, usize),
    OrI(usize, i64, usize),
    SetR(usize, usize),
    SetI(i64, usize),
    GtIR(i64, usize, usize),
    GtRI(usize, i64, usize),
    GtRR(usize, usize, usize),
    EqIR(i64, usize, usize),
    EqRI(usize, i64, usize),
    EqRR(usize, usize, usize),
}

fn run_instr(state: &mut CpuState, instr: Instr) {
    match instr {
        Instr::AddR(a, b, c) => state[c] = state[a] + state[b],
        Instr::AddI(a, b, c) => state[c] = state[a] + b,
        Instr::MulR(a, b, c) => state[c] = state[a] * state[b],
        Instr::MulI(a, b, c) => state[c] = state[a] * b,
        Instr::AndR(a, b, c) => state[c] = state[a] & state[b],
        Instr::AndI(a, b, c) => state[c] = state[a] & b,
        Instr::OrR(a, b, c) => state[c] = state[a] | state[b],
        Instr::OrI(a, b, c) => state[c] = state[a] | b,
        Instr::SetR(a, c) => state[c] = state[a],
        Instr::SetI(a, c) => state[c] = a,
        Instr::GtIR(a, b, c) => state[c] = if a > state[b] { 1 } else { 0 },
        Instr::GtRI(a, b, c) => state[c] = if state[a] > b { 1 } else { 0 },
        Instr::GtRR(a, b, c) => state[c] = if state[a] > state[b as usize] { 1 } else { 0 },
        Instr::EqIR(a, b, c) => state[c] = if a == state[b] { 1 } else { 0 },
        Instr::EqRI(a, b, c) => state[c] = if state[a] == b { 1 } else { 0 },
        Instr::EqRR(a, b, c) => state[c] = if state[a] == state[b] { 1 } else { 0 },
    }
}

fn run_prog(state: &mut CpuState, prog: &Vec<Instr>, pc: usize) {
    while state[pc] != 1 {
        run_instr(state, prog[state[pc] as usize]);
        state[pc] += 1;
    }
}

fn parse_instr(s: &str) -> Option<Instr> {
    let splt = s.split(" ").collect::<Vec<_>>();
    let vals = splt.iter().skip(1)
        .map(|x| x.parse::<i64>().ok())
        .collect::<Vec<_>>();
    let regs = splt.iter().skip(1)
        .map(|x| x.parse::<usize>().ok())
        .collect::<Vec<_>>();
    return match splt[0] {
        "addr" => Some(Instr::AddR(regs[0]?, regs[1]?, regs[2]?)),
        "addi" => Some(Instr::AddI(regs[0]?, vals[1]?, regs[2]?)),
        "mulr" => Some(Instr::MulR(regs[0]?, regs[1]?, regs[2]?)),
        "muli" => Some(Instr::MulI(regs[0]?, vals[1]?, regs[2]?)),
        "banr" => Some(Instr::AndR(regs[0]?, regs[1]?, regs[2]?)),
        "bani" => Some(Instr::AndI(regs[0]?, vals[1]?, regs[2]?)),
        "borr" => Some(Instr::OrR(regs[0]?, regs[1]?, regs[2]?)),
        "bori" => Some(Instr::OrI(regs[0]?, vals[1]?, regs[2]?)),
        "setr" => Some(Instr::SetR(regs[0]?, regs[2]?)),
        "seti" => Some(Instr::SetI(vals[0]?, regs[2]?)),
        "gtir" => Some(Instr::GtIR(vals[0]?, regs[1]?, regs[2]?)),
        "gtri" => Some(Instr::GtRI(regs[0]?, vals[1]?, regs[2]?)),
        "gtrr" => Some(Instr::GtRR(regs[0]?, regs[1]?, regs[2]?)),
        "eqir" => Some(Instr::EqIR(vals[0]?, regs[1]?, regs[2]?)),
        "eqri" => Some(Instr::EqRI(regs[0]?, vals[1]?, regs[2]?)),
        "eqrr" => Some(Instr::EqRR(regs[0]?, regs[1]?, regs[2]?)),
        _ => None,
    }
}

fn sum_div(n: i64) -> i64 {
    let mut sum = 0;
    let mut i = 1;
    while i*i <= n {
        if n % i == 0 {
            sum += i;
            if i*i != n {
                sum += n / i;
            }
        }
        i += 1;
    }
    return sum;
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines()
        .map(|x| x.unwrap())
        .collect::<Vec<_>>();
    let pc = lines[0].split(" ").skip(1).next().unwrap().parse::<usize>().unwrap();
    let mut prog = Vec::new();
    for i in 1..lines.len() {
        prog.push(parse_instr(&lines[i]).unwrap());
    }
    let mut state = [0; 6];
    state[0] = 1;
    run_prog(&mut state, &prog, pc);
    println!("Result: {}", sum_div(state[5]));
}

