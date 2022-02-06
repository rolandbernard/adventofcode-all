
use std::io;
use std::io::prelude::*;

fn sum_meta(nums: &Vec<usize>, mut off: usize) -> (usize, usize) {
    let mut sum = 0;
    let childs = nums[off];
    let metas = nums[off + 1];
    off += 2;
    for _ in 0..childs {
        let (ds, no) = sum_meta(nums, off);
        sum += ds;
        off = no;
    }
    for i in 0..metas {
        sum += nums[off + i];
    }
    return (sum, off + metas);
}

fn main() {
    let stdin = io::stdin();
    let file = stdin.lock().lines().next().unwrap().unwrap();
    let numbers = file.split(" ")
        .map(|x| x.parse::<usize>().unwrap()) 
        .collect::<Vec<usize>>();
    println!("Result: {}", sum_meta(&numbers, 0).0);
}

