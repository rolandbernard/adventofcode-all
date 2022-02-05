
use std::io;
use std::io::prelude::*;

const N: usize = 5;
const D: usize = 60;

fn main() {
    let stdin = io::stdin();
    let instr = stdin.lock().lines()
        .map(|x| {
            let line = x.unwrap().into_bytes();
            ((line[5] - 'A' as u8) as usize, (line[36] - 'A' as u8) as usize)
        })
        .collect::<Vec<(usize, usize)>>();
    let mut req = vec![0; 26];
    let mut next = vec![Vec::new(); 26];
    for inst in &instr {
        req[inst.1] += 1;
        next[inst.0].push(inst.1);
    }
    let mut queue = Vec::new();
    for i in 0..26 {
        if next[i].len() != 0 && req[i] == 0 {
            queue.push(i);
        }
    }
    let mut time = 0;
    let mut work = [0; N];
    let mut doing = [usize::MAX; N];
    while !queue.is_empty() || *work.iter().max().unwrap() > 0 {
        let mut min = 0;
        if queue.is_empty() {
            for i in 0..work.len() {
                if (work[i] != 0 && work[i] < work[min]) || work[min] == 0 {
                    min = i;
                }
            }
        } else {
            for i in 0..work.len() {
                if work[i] < work[min] {
                    min = i;
                }
            }
        }
        let dt = work[min];
        time += dt;
        for i in 0..work.len() {
            if work[i] != 0 {
                work[i] -= dt;
                if work[i] == 0 && doing[i] != usize::MAX {
                    for &r in &next[doing[i]] {
                        req[r] -= 1;
                        if req[r] == 0 {
                            queue.push(r);
                        }
                    }
                    doing[i] = usize::MAX;
                }
            }
        }
        if !queue.is_empty() {
            queue.sort_by(|a, b| b.cmp(a));
            let n = queue.pop().unwrap();
            doing[min] = n;
            work[min] = D + 1 + n;
        }
    }
    println!("Result: {}", time);
}

