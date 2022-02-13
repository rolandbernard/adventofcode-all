
use std::io;
use std::io::prelude::*;

#[path ="../intcode/intcode.rs"]
mod intcode;

fn run_amp(mut mem: intcode::IntMem, set: i64, inp: i64) -> i64 {
    let mut cpu = intcode::IntState::new();
    let inp_arr = [set, inp];
    let mut inp_iter = inp_arr.iter();
    let mut out = None;
    while !cpu.halt && out.is_none() {
        intcode::run_instr(&mut mem, &mut cpu, || *inp_iter.next().unwrap(), |x| {
            out = Some(x);
        });
    }
    return out.unwrap();
}

fn run_all(mem: &intcode::IntMem, set: &[i64; 5]) -> i64 {
    let mut last = 0;
    for i in 0..5 {
        last = run_amp(mem.clone(), set[i], last);
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
    let mut set = [0, 1, 2, 3, 4];
    for _ in 0..120 {
        max = max.max(run_all(&mem, &set));
        next_permutation(&mut set);
    }
    println!("Result: {}", max);
}

