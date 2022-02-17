
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
    let mut unsent = vec![Vec::new(); N];
    let queues = RefCell::new(vec![VecDeque::new(); N]);
    let mut res = None;
    while res == None {
        for i in 0..N {
            intcode::run_instr(
                &mut mems[i], &mut cpus[i],
                || queues.borrow_mut()[i].pop_front().unwrap_or(-1),
                |x| {
                    unsent[i].push(x);
                    if unsent[i].len() == 3 {
                        if unsent[i][0] == 255 {
                            res = Some(unsent[i][2]);
                        } else {
                            let j = unsent[i][0] as usize;
                            queues.borrow_mut()[j].push_back(unsent[i][1]);
                            queues.borrow_mut()[j].push_back(unsent[i][2]);
                        }
                        unsent[i].clear();
                    }
                }
            );
        }
    }
    println!("Result: {}", res.unwrap());
}

