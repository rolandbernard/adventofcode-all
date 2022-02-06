
use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let splt = line.split(" ").collect::<Vec<&str>>();
    let players = splt[0].parse::<usize>().unwrap();
    let points = splt[6].parse::<usize>().unwrap();
    let mut scores = vec![0; players];
    let mut marbels = vec![0];
    let mut current = 0;
    for i in 1..=points {
        let player = (i - 1) % players;
        if i % 23 == 0 {
            current = (current + marbels.len() - 7) % marbels.len();
            scores[player] += i + marbels[current];
            marbels.remove(current);
        } else {
            current = (current + 2) % marbels.len();
            marbels.insert(current, i);
        }
    }
    println!("Result: {}", scores.iter().max().unwrap());
}

