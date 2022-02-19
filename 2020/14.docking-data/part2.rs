
use std::io;
use std::io::prelude::*;
use std::collections::*;

fn write_all(mem: &mut HashMap<usize, usize>, addr: usize, mask: &str, val: usize, d: usize) {
    if d == mask.len() {
        mem.insert(addr, val);
    } else {
        let c = mask.chars().skip(d).next().unwrap();
        if c == '0' {
            write_all(mem, addr, mask, val, d + 1);
        } else if c == '1' {
            write_all(mem, addr | (1 << (mask.len() - d - 1)), mask, val, d + 1);
        } else if c == 'X' {
            write_all(mem, addr & !(1 << (mask.len() - d - 1)), mask, val, d + 1);
            write_all(mem, addr | (1 << (mask.len() - d - 1)), mask, val, d + 1);
        } else {
            panic!();
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines()
        .map(|l| l.unwrap())
        .collect::<Vec<_>>();
    let mut mask = String::new();
    let mut mem = HashMap::new();
    for l in lines.iter() {
        let splt = l.split(" = ").collect::<Vec<_>>();
        if splt[0] == "mask" {
            mask = splt[1].to_owned();
        } else {
            let nums = splt.into_iter()
                .map(|x| x.trim_matches(|c: char| !c.is_digit(10)).parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            write_all(&mut mem, nums[0], &mask, nums[1], 0);
        }
    }
    println!("Result: {}", mem.values().sum::<usize>());
}

