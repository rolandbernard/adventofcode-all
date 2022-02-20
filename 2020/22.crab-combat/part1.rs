
use std::io;
use std::io::prelude::*;
use std::collections::*;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|x| x.unwrap()).collect::<Vec<_>>();
    let split = lines.iter().position(|l| l == "").unwrap();
    let mut stack0 = lines.iter().skip(1).take(split - 1)
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<VecDeque<_>>();
    let mut stack1 = lines.iter().skip(split + 2)
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<VecDeque<_>>();
    while !stack0.is_empty() && !stack1.is_empty() {
        let card0 = stack0.pop_front().unwrap();
        let card1 = stack1.pop_front().unwrap();
        if card0 > card1 {
            stack0.push_back(card0);
            stack0.push_back(card1);
        } else if card1 > card0 {
            stack1.push_back(card1);
            stack1.push_back(card0);
        } else {
            panic!();
        }
    }
    let winner;
    if stack1.is_empty() {
        winner = stack0;
    } else {
        winner = stack1;
    }
    let score = winner.iter().enumerate().map(|(i, &x)| x * (winner.len() - i)).sum::<usize>();
    println!("Result: {}", score);
}

