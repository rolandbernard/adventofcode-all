
use std::io;
use std::io::prelude::*;

fn apply_tail(n: usize, nums: &mut Vec<i64>) {
    let len = nums.len();
    for i in 2..=n {
        nums[len - i] = (nums[len - i] + nums[len - i + 1]).abs() % 10;
    }
}

fn main() {
    let stdin = io::stdin();
    let part = stdin.lock().lines().next().unwrap().unwrap()
        .bytes().map(|x| (x - '0' as u8) as i64).collect::<Vec<_>>();
    let len = part.len() * 10000;
    let mut numbers = part.into_iter().cycle().take(len).collect::<Vec<i64>>();
    let skip = numbers.iter().take(7).fold(0, |val, i| 10*val + i);
    let tail = len - skip as usize;
    for _ in 0..100 {
        apply_tail(tail, &mut numbers);
    }
    println!("Result: {}", (skip..skip + 8).map(|x| numbers[x as usize]).map(|x| (x as u8 + '0' as u8) as char).collect::<String>());
}

