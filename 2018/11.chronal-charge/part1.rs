
use std::io;
use std::io::prelude::*;

fn fuel(pos: (i64, i64), s: i64) -> i64 {
    let rack_id = pos.0 + 10;
    let power = rack_id * (rack_id * pos.1 + s);
    return (power / 100) % 10 - 5;
}

fn fuel3x3(pos: (i64, i64), s: i64) -> i64 {
    let mut sum = 0;
    for i in 0..3 {
        for j in 0..3 {
            sum += fuel((pos.0 + i, pos.1 + j), s);
        }
    }
    return sum;
}

fn main() {
    let stdin = io::stdin();
    let ser = stdin.lock().lines().next().unwrap().unwrap()
        .parse::<i64>().unwrap();
    let mut max = (1, 1);
    for x in 1..=(300 - 3) {
        for y in 1..=(300 - 3) {
            if fuel3x3((x, y), ser) > fuel3x3(max, ser) {
                max = (x, y);
            }
        }
    }
    println!("Result: {},{}", max.0, max.1);
}

