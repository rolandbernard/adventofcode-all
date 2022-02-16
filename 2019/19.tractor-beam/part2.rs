
use std::io;
use std::io::prelude::*;

#[path ="../intcode/intcode.rs"]
mod intcode;

const SIZE: i64 = 100;

fn value_at(mut mem: intcode::IntMem, pos: (i64, i64)) -> i64 {
    let mut out = None;
    let inp_arr = [pos.0, pos.1];
    let mut inp = inp_arr.iter();
    let mut cpu = intcode::IntState::new();
    while !cpu.halt && out == None {
        intcode::run_instr(&mut mem, &mut cpu, || *inp.next().unwrap(), |x| out = Some(x));
    }
    return out.unwrap_or(0);
}

fn main() {
    let stdin = io::stdin();
    let mem = intcode::parse(&stdin.lock().lines().next().unwrap().unwrap());
    let mut first = 0;
    let mut last = 0;
    let res = 'result: loop {
        for i in 10.. {
            while value_at(mem.clone(), (i, first)) == 0 {
                first += 1;
            }
            last = last.max(first);
            while value_at(mem.clone(), (i, last)) == 1 {
                last += 1;
            }
            if first + SIZE < last {
                let x = last - SIZE;
                if value_at(mem.clone(), (i + SIZE - 1, x)) == 1 {
                    break 'result 10000 * i + x;
                }
            }
        }
    };
    println!("Result: {}", res);
}

