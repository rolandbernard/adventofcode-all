
use std::io;
use std::io::prelude::*;
use std::collections::*;
use std::cell::*;

#[path ="../intcode/intcode.rs"]
mod intcode;

fn main() {
    let stdin = io::stdin();
    let mut mem = intcode::parse(&stdin.lock().lines().next().unwrap().unwrap());
    mem[0] = 2;
    let mut cpu = intcode::IntState::new();
    let mut vals = Vec::new();
    let mut screen = HashMap::new();
    let mut score = 0;
    let paddle = Cell::new((0, 0));
    let ball = Cell::new((0, 0));
    while !cpu.halt {
        let mut inp = false;
        intcode::run_instr(&mut mem, &mut cpu, || {
            inp = true;
            if ball.get().0 < paddle.get().0 {
                return -1;
            } else if ball.get().0 > paddle.get().0 {
                return 1;
            } else {
                return 0;
            }
        }, |x| {
            vals.push(x);
            if vals.len() == 3 {
                if (vals[0], vals[1]) == (-1, 0) {
                    score = vals[2];
                } else if vals[2] == 3 {
                    paddle.set((vals[0], vals[1]));
                } else if vals[2] == 4 {
                    ball.set((vals[0], vals[1]));
                }
                screen.insert((vals[0], vals[1]), vals[2]);
                vals.clear();
            }
        });
    }
    println!("Result: {}", score);
}

