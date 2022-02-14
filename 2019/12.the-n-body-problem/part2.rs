
use std::io;
use std::io::prelude::*;
use std::collections::*;

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    return a;
}

fn lcm(a: i64, b: i64) -> i64 {
    return a * b / gcd(a, b);
}

fn main() {
    let stdin = io::stdin();
    let mut moons = vec![Vec::new(); 3];
    for l in stdin.lock().lines() {
        let line = l.unwrap();
        let pos = line[1..line.len() - 1].split(", ")
            .map(|x| x.split("=").skip(1).next().unwrap().parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        for i in 0..3 {
            moons[i].push((pos[i], 0));
        }
    }
    let mut period = 1;
    for x in 0..3 {
        let mut seen = HashMap::new();
        seen.insert(moons[x].clone(), 0);
        let mut i = 0;
        let rep = loop {
            for i in 0..moons[x].len() {
                for j in 0..moons[x].len() {
                    if moons[x][i].0 > moons[x][j].0 {
                        moons[x][i].1 -= 1;
                    } else if moons[x][i].0 < moons[x][j].0 {
                        moons[x][i].1 += 1;
                    }
                }
            }
            for i in 0..moons[x].len() {
                moons[x][i].0 += moons[x][i].1;
            }
            i += 1;
            if seen.contains_key(&moons[x]) {
                break i - seen[&moons[x]];
            } else {
                seen.insert(moons[x].clone(), i);
            }
        };
        period = lcm(period, rep);
    }
    println!("Result: {}", period);
}

