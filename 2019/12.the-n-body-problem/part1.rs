
use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let mut moons = Vec::new();
    for l in stdin.lock().lines() {
        let line = l.unwrap();
        let pos = line[1..line.len() - 1].split(", ")
            .map(|x| x.split("=").skip(1).next().unwrap().parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        moons.push(([pos[0], pos[1], pos[2]], [0, 0, 0]));
    }
    for _ in 0..1000 {
        for i in 0..moons.len() {
            for j in 0..moons.len() {
                for x in 0..3 {
                    if moons[i].0[x] > moons[j].0[x] {
                        moons[i].1[x] -= 1;
                    } else if moons[i].0[x] < moons[j].0[x] {
                        moons[i].1[x] += 1;
                    }
                }
            }
        }
        for i in 0..moons.len() {
            for x in 0..3 {
                moons[i].0[x] += moons[i].1[x];
            }
        }
    }
    let energy = moons.iter()
        .map(|x| x.0.iter().map(|x| x.abs()).sum::<i64>() * x.1.iter().map(|x| x.abs()).sum::<i64>())
        .sum::<i64>();
    println!("Result: {}", energy);
}

