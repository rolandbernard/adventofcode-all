
use std::io;
use std::io::prelude::*;
use std::collections::*;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines()
        .map(|l| l.unwrap())
        .collect::<Vec<_>>();
    let mut or = 0;
    let mut and = i64::MAX;
    let mut mem = HashMap::new();
    for l in lines.iter() {
        let splt = l.split(" = ").collect::<Vec<_>>();
        if splt[0] == "mask" {
            or = splt[1].chars().map(|x| if x == '1' {1} else {0}).fold(0, |a, x| 2*a + x);
            and = splt[1].chars().map(|x| if x == '0' {0} else {1}).fold(0, |a, x| 2*a + x);
        } else {
            let nums = splt.into_iter()
                .map(|x| x.trim_matches(|c: char| !c.is_digit(10)).parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            mem.insert(nums[0], (nums[1] | or) & and);
        }
    }
    println!("Result: {}", mem.values().sum::<i64>());
}

