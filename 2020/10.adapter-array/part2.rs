
use std::io;
use std::io::prelude::*;

fn count_pos(i: usize, nums: &Vec<usize>, cache: &mut Vec<usize>) -> usize {
    if cache[i] == 0 {
        let mut sum = 0;
        let mut j = i + 1;
        while j < nums.len() && nums[i] + 3 >= nums[j] {
            sum += count_pos(j, nums, cache);
            j += 1;
        }
        cache[i] = sum;
    }
    return cache[i];
}

fn main() {
    let stdin = io::stdin();
    let mut nums = stdin.lock().lines()
        .map(|l| l.unwrap().parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    nums.push(0);
    nums.push(nums.iter().max().unwrap() + 3);
    nums.sort();
    let mut cache = vec![0; nums.len()];
    cache[nums.len() - 1] = 1;
    println!("Result: {}", count_pos(0, &nums, &mut cache));
}

