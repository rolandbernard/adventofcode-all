
use std::io;
use std::io::prelude::*;

fn evaluate(s: &str) -> i64 {
    let mut acc = vec![0];
    let mut op = vec!['+'];
    let mut val = None;
    for c in s.chars().chain(" ".chars()) {
        let len = acc.len();
        if !c.is_digit(10) && val != None {
            if op[len - 1] == '+' {
                acc[len - 1] += val.unwrap();
            } else {
                acc[len - 1] *= val.unwrap();
            }
            val = None;
        }
        match c {
            '(' => {
                acc.push(0);
                op.push('+');
            },
            ')' => {
                val = Some(acc.pop().unwrap());
                op.pop();
            },
            '+' => op[len - 1] = '+',
            '*' => op[len - 1] = '*',
            c if c.is_digit(10) => {
                val = Some(10 * val.unwrap_or(0) + c.to_digit(10).unwrap() as i64);
            },
            ' ' => {},
            _ => panic!(),
        }
    }
    return acc[0];
}

fn main() {
    let stdin = io::stdin();
    let mut sum = 0;
    for l in stdin.lock().lines() {
        let line = l.unwrap();
        sum += evaluate(&line);
    }
    println!("Result: {}", sum);
}

