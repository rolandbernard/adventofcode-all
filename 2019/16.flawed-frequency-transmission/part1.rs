
use std::io;
use std::io::prelude::*;

fn fft_element(off: usize, nums: &Vec<i64>) -> i64 {
    const PAT: [i64; 4] = [0, 1, 0, -1];
    let mut phase = 0;
    let mut skip = 1;
    let mut sum = 0;
    for &x in nums {
        if skip == off + 1 {
            phase += 1;
            skip = 0;
            if phase >= PAT.len() {
                phase = 0;
            }
        }
        sum += x * PAT[phase];
        skip += 1;
    }
    return sum.abs() % 10;
}

fn fft(nums: Vec<i64>) -> Vec<i64> {
    let mut res = vec![0; nums.len()];
    for i in 0..nums.len() {
        res[i] = fft_element(i, &nums);
    }
    return res;
}

fn main() {
    let stdin = io::stdin();
    let mut numbers = stdin.lock().lines().next().unwrap().unwrap()
        .bytes().map(|x| (x - '0' as u8) as i64).collect::<Vec<_>>();
    for _ in 0..100 {
        numbers = fft(numbers);
    }
    println!("Result: {}", numbers.iter().take(8).map(|&x| (x as u8 + '0' as u8) as char).collect::<String>());
}

