
use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines()
        .map(|x| x.unwrap())
        .collect::<Vec<_>>();
    let depth = lines[0].split(" ").skip(1).next().unwrap()
        .parse::<usize>().unwrap();
    let target = lines[1].split(" ").skip(1).next().unwrap().split(",")
        .map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
    let mut map = vec![vec![0; target[1] + 1]; target[0] + 1];
    for i in 1..=target[0] {
        map[i][0] = (i * 16807 + depth) % 20183;
    }
    for i in 1..=target[1] {
        map[0][i] = (i * 48271 + depth) % 20183;
    }
    for i in 1..=target[0] {
        for j in 1..=target[1] {
            map[i][j] = (map[i - 1][j] * map[i][j - 1] + depth) % 20183;
        }
    }
    map[target[0]][target[1]] = 0;
    for i in 0..=target[0] {
        for j in 0..=target[1] {
            map[i][j] %= 3;
        }
    }
    println!("Result: {}", map.iter().flatten().sum::<usize>());
}

