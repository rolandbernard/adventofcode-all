
use std::io;
use std::io::prelude::*;

fn fuel(pos: (usize,usize), s: i64) -> i64 {
    let rack_id = pos.0 as i64 + 10;
    let power = rack_id * (rack_id * pos.1 as i64 + s);
    return (power / 100) % 10 - 5;
}

fn fuel_square(pos: (usize, usize, usize), fuelz: &Vec<Vec<i64>>) -> i64 {
    return fuelz[pos.0 + pos.2 - 1][pos.1 + pos.2 - 1] + fuelz[pos.0 - 1][pos.1 - 1] - 
        fuelz[pos.0 + pos.2 - 1][pos.1 - 1] - fuelz[pos.0 - 1][pos.1 + pos.2 - 1];
}

fn main() {
    let stdin = io::stdin();
    let ser = stdin.lock().lines().next().unwrap().unwrap()
        .parse::<i64>().unwrap();
    let mut fuelz = vec![vec![0; 301]; 301];
    for x in 1..=300 {
        for y in 1..=300 {
            fuelz[x][y] = fuel((x, y), ser) + fuelz[x - 1][y]
                + fuelz[x][y - 1] - fuelz[x - 1][y - 1];
        }
    }
    let mut best = (1, 1, 1);
    let mut max = 0;
    for x in 1..=300 {
        for y in 1..=300 {
            for s in 1..=((301 - x).min(301 - y)) {
                let f = fuel_square((x, y, s), &fuelz);
                if f > max {
                    max = f;
                    best = (x, y, s);
                }
            }
        }
    }
    println!("Result: {},{},{}", best.0, best.1, best.2);
}

