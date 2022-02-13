
use std::io;
use std::io::prelude::*;

#[path ="../intcode/intcode.rs"]
mod intcode;

fn init_amp(mem: &mut intcode::IntMem, cpu: &mut intcode::IntState, set: i64) {
    let mut halt = false;
    while !cpu.halt && !halt {
        intcode::run_instr(mem, cpu, || {halt = true; set}, |_| {});
    }
}

fn run_amp(mem: &mut intcode::IntMem, cpu: &mut intcode::IntState, inp: i64) -> i64 {
    let mut out = None;
    while !cpu.halt && out.is_none() {
        intcode::run_instr(mem, cpu, || inp, |x| { out = Some(x); });
    }
    return out.unwrap_or(inp);
}

fn run_all(mem: &intcode::IntMem, set: &[i64; 5]) -> i64 {
    let mut mems = vec![mem.clone(); 5];
    let mut cpus = vec![intcode::IntState::new(); 5];
    for i in 0..5 {
        init_amp(&mut mems[i], &mut cpus[i], set[i]);
    }
    let mut last = 0;
    while !cpus[4].halt {
        for i in 0..5 {
            last = run_amp(&mut mems[i], &mut cpus[i], last);
        }
    }
    return last;
}

fn next_permutation(array: &mut [i64]) {
    let mut i = array.len() - 1;
    while i > 0 && array[i] <= array[i - 1] {
        i -= 1;
    }
    if i == 0 {
        array[..].reverse();
    } else {
        let mut floor = array.len() - 1;
        while array[floor] <= array[i - 1] {
            floor -= 1;
        }
        array.swap(floor, i - 1);
        array[i..].reverse();
    }
}

fn main() {
    let stdin = io::stdin();
    let mem = intcode::parse(&stdin.lock().lines().next().unwrap().unwrap());
    let mut max = 0;
    let mut set = [5, 6, 7, 8, 9];
    for _ in 0..120 {
        max = max.max(run_all(&mem, &set));
        next_permutation(&mut set);
    }
    println!("Result: {}", max);
}

