
use std::io;
use std::io::prelude::*;

fn is_valid(mut i: i64) -> bool {
    let mut last = 10;
    let mut dub = false;
    while i != 0 {
        let dig = i % 10;
        if dig > last {
            return false;
        } else if dig == last {
            dub = true;
        }
        i /= 10;
        last = dig;
    }
    return dub;
}

fn main() {
    let stdin = io::stdin();
    let range = stdin.lock().lines().next().unwrap().unwrap()
        .split("-").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();
    let mut count = 0;
    for i in range[0]..=range[1] {
        if is_valid(i) {
            count += 1;
        }
    }
    println!("Result: {}", count);
}

