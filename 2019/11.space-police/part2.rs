
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
    let painted = RefCell::new(HashSet::new());
    painted.borrow_mut().insert((0, 0));
    let pos = Cell::new((0, 0));
    let mut dir = (-1, 0);
    let mut paint = true;
    while !cpu.halt {
        intcode::run_instr(&mut mem, &mut cpu, || {
            return if painted.borrow().contains(&pos.get()) { 1 } else { 0 };
        }, |x| {
            if paint {
                if x == 1 {
                    painted.borrow_mut().insert(pos.get());
                } else {
                    painted.borrow_mut().remove(&pos.get());
                }
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
    println!("Result:");
    let top = painted.borrow().iter().map(|x| x.0).min().unwrap();
    let bottom = painted.borrow().iter().map(|x| x.0).max().unwrap();
    let left = painted.borrow().iter().map(|x| x.1).min().unwrap();
    let right = painted.borrow().iter().map(|x| x.1).max().unwrap();
    for i in top..=bottom {
        for j in left..=right {
            print!("{}", if painted.borrow().contains(&(i, j)) {'#'} else {' '});
        }
        println!();
    }
}

