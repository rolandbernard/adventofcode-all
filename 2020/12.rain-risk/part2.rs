
use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let mut pos = (0, 0);
    let mut dir = (-1, -10);
    for l in stdin.lock().lines() {
        let line = l.unwrap();
        let num = line[1..].parse::<i64>().unwrap();
        match line.chars().next().unwrap() {
            'N' => dir.0 -= num,
            'S' => dir.0 += num,
            'E' => dir.1 -= num,
            'W' => dir.1 += num,
            'L' => for _ in 0..num/90 { dir = (dir.1, -dir.0) },
            'R' => for _ in 0..num/90 { dir = (-dir.1, dir.0) },
            'F' => pos = (pos.0 + num * dir.0, pos.1 + num * dir.1),
            _ => panic!(),
        }
    }
    println!("Result: {}", pos.0.abs() + pos.1.abs());
}

