
use std::io;
use std::io::prelude::*;
use std::collections::*;

const N: usize = 10007;

fn main() {
    let stdin = io::stdin();
    let mut stack = (0..N).collect::<VecDeque<_>>();
    for l in stdin.lock().lines() {
        let line = l.unwrap();
        let splt = line.split(" ").collect::<Vec<_>>();
        if splt[0] == "deal" && splt[1] == "into" {
            let mut new = VecDeque::with_capacity(N);
            for v in stack {
                new.push_front(v);
            }
            stack = new;
        } else if splt[0] == "cut" {
            let n = splt[1].parse::<i64>().unwrap();
            if n < 0 {
                for _ in 0..-n {
                    let val = stack.pop_back().unwrap();
                    stack.push_front(val);
                }
            } else {
                for _ in 0..n {
                    let val = stack.pop_front().unwrap();
                    stack.push_back(val);
                }
            }
        } else {
            let n = splt[3].parse::<usize>().unwrap();
            let mut new = (0..N).collect::<VecDeque<_>>();
            let mut i = 0;
            for v in stack {
                new[i % N] = v;
                i += n;
            }
            stack = new;
        }
    }
    println!("Result: {}", stack.iter().position(|&x| x == 2019).unwrap());
}

