
use std::io;
use std::io::prelude::*;

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
    let mut order = Vec::new();
    while !queue.is_empty() {
        queue.sort_by(|a, b| b.cmp(a));
        let n = queue.pop().unwrap();
        order.push(n);
        for &r in &next[n] {
            req[r] -= 1;
            if req[r] == 0 {
                queue.push(r);
            }
        }
    }
    println!("Result: {}", order.iter().map(|&x| (x as u8 + 'A' as u8) as char).collect::<String>());
}

