
use std::io;
use std::io::prelude::*;
use std::collections::*;
use std::cell::*;

#[path ="../intcode/intcode.rs"]
mod intcode;

fn main() {
    let stdin = io::stdin();
    let mut mem = intcode::parse(&stdin.lock().lines().next().unwrap().unwrap());
    let mut cpu = intcode::IntState::new();
    let painted = RefCell::new(HashMap::new());
    let pos = Cell::new((0, 0));
    let mut dir = (-1, 0);
    let mut paint = true;
    while !cpu.halt {
        intcode::run_instr(&mut mem, &mut cpu, || {
            return *painted.borrow().get(&pos.get()).unwrap_or(&0);
        }, |x| {
            if paint {
                painted.borrow_mut().insert(pos.get(), x);
            } else {
                if x == 0 {
                    dir = (-dir.1, dir.0);
                } else {
                    dir = (dir.1, -dir.0);
                }
                let c = pos.get();
                pos.set((c.0 + dir.0, c.1 + dir.1));
            }
            paint = !paint;
        });
    }
    println!("Result: {}", painted.borrow().len());
}

