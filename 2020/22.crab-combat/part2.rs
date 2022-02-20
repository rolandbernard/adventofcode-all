
use std::io;
use std::io::prelude::*;
use std::collections::*;

fn score(stack: &VecDeque<usize>) -> usize {
    return stack.iter().enumerate()
        .map(|(i, &x)| x * (stack.len() - i))
        .sum::<usize>();
}

type GameState = (VecDeque<usize>, VecDeque<usize>);

fn combat(mut state: GameState) -> (bool, usize) {
    let mut seen = HashSet::new();
    while !state.0.is_empty() && !state.1.is_empty() && !seen.contains(&state) {
        seen.insert(state.clone());
        let card0 = state.0.pop_front().unwrap();
        let card1 = state.1.pop_front().unwrap();
        let winner;
        if card0 <= state.0.len() && card1 <= state.1.len() {
            let sub0 = state.0.iter().map(|&x| x).take(card0).collect::<VecDeque<_>>();
            let sub1 = state.1.iter().map(|&x| x).take(card1).collect::<VecDeque<_>>();
            let (w, _) = combat((sub0, sub1));
            winner = w;
        } else {
            winner = card0 > card1;
        }
        if winner {
            state.0.push_back(card0);
            state.0.push_back(card1);
        } else {
            state.1.push_back(card1);
            state.1.push_back(card0);
        }
    }
    if state.0.is_empty() {
        return (false, score(&state.1));
    } else {
        return (true, score(&state.0));
    }
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|x| x.unwrap()).collect::<Vec<_>>();
    let split = lines.iter().position(|l| l == "").unwrap();
    let stack0 = lines.iter().skip(1).take(split - 1)
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<VecDeque<_>>();
    let stack1 = lines.iter().skip(split + 2)
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<VecDeque<_>>();
    let (_, score) = combat((stack0, stack1));
    println!("Result: {}", score);
}

