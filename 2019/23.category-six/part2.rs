
use std::io;
use std::io::prelude::*;
use std::collections::*;
use std::cell::*;

#[path ="../intcode/intcode.rs"]
mod intcode;

const N: usize = 50;

fn main() {
    let stdin = io::stdin();
    let mem = intcode::parse(&stdin.lock().lines().next().unwrap().unwrap());
    let mut mems = vec![mem.clone(); N];
    let mut cpus = vec![intcode::IntState::new(); N];
    for i in 0..N {
        let mut end = false;
        while !end {
            intcode::run_instr(&mut mems[i], &mut cpus[i], || {end = true; i as i64}, |_| {});
        }
    }
    let mut idle = vec![false; N];
    let mut unsent = vec![Vec::new(); N];
    let queues = RefCell::new(vec![VecDeque::new(); N]);
    let mut res = None;
    let mut nat = None;
    let mut last = None;
    while res == None {
        let mut all_idle = true;
        for i in 0..N {
            intcode::run_instr(
                &mut mems[i], &mut cpus[i],
                || {
                    let v = queues.borrow_mut()[i].pop_front().unwrap_or(-1);
                    idle[i] = v == -1;
                    return v;
                },
                |x| {
                    unsent[i].push(x);
                    if unsent[i].len() == 3 {
                        if unsent[i][0] == 255 {
                            nat = Some((unsent[i][1], unsent[i][2]));
                        } else {
                            let j = unsent[i][0] as usize;
                            queues.borrow_mut()[j].push_back(unsent[i][1]);
                            queues.borrow_mut()[j].push_back(unsent[i][2]);
                        }
                        unsent[i].clear();
                    }
                }
            );
            if !idle[i] {
                all_idle = false;
            }
        }
        if all_idle {
            if let Some((x, y)) = nat {
                if let Some((_, l)) = last {
                    if l == y {
                        res = Some(l);
                    }
                }
                queues.borrow_mut()[0].push_back(x);
                queues.borrow_mut()[0].push_back(y);
                idle[0] = false;
                last = nat;
            }
        }
    }
    println!("Result: {}", res.unwrap());
}

