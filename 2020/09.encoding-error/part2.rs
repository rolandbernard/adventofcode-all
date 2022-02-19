
use std::io;
use std::io::prelude::*;
use std::collections::*;

fn main() {
    let stdin = io::stdin();
    let nums = stdin.lock().lines()
        .map(|l| l.unwrap().parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let mut inv = 0;
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
            inv = nums[i];
            break;
        }
    }
    let mut i = 0;
    let mut j = 0;
    let mut sum = 0;
    while sum > inv || j < nums.len() {
        if sum < inv || j - i < 2 {
            sum += nums[j];
            j += 1;
        } else if sum > inv {
            sum -= nums[i];
            i += 1;
        } else {
            break;
        }
    }
    let min = nums.iter().skip(i).take(j - i).min().unwrap();
    let max = nums.iter().skip(i).take(j - i).max().unwrap();
    println!("Result: {}", min + max);
}

