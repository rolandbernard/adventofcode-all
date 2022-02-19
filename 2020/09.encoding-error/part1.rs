
use std::io;
use std::io::prelude::*;
use std::collections::*;

fn main() {
    let stdin = io::stdin();
    let nums = stdin.lock().lines()
        .map(|l| l.unwrap().parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let mut res = 0;
    for i in 25..nums.len() {
        let mut seen = HashSet::new();
        let mut valid = false;
        for j in i - 25..i {
            if seen.contains(&(nums[i] - nums[j])) {
                valid = true;
                break;
            }
            seen.insert(nums[j]);
        }
        if !valid {
            res = nums[i];
            break;
        }
    }
    println!("Result: {}", res);
}

