
use std::io;
use std::io::prelude::*;

fn evaluate_base(s: &Vec<char>, mut off: usize) -> (usize, i64) {
    while off < s.len() && s[off].is_whitespace() { off += 1 }
    if off < s.len() && s[off] == '(' {
        let (mut off, res) = evaluate_mul(s, off + 1);
        while off < s.len() && s[off].is_whitespace() { off += 1 }
        return (off + 1, res);
    } else {
        let mut val = 0;
        while off < s.len() && s[off].is_digit(10) {
            val = val * 10 + s[off].to_digit(10).unwrap() as i64;
            off += 1;
        }
        return (off, val);
    }
}

fn evaluate_add(s: &Vec<char>, mut off: usize) -> (usize, i64) {
    while off < s.len() && s[off].is_whitespace() { off += 1 }
    let (mut off, mut res) = evaluate_base(s, off);
    while off < s.len() && s[off].is_whitespace() { off += 1 }
    while off < s.len() && s[off] == '+' {
        let (noff, sub) = evaluate_base(s, off + 1);
        off = noff;
        res += sub;
        while off < s.len() && s[off].is_whitespace() { off += 1 }
    }
    return (off, res);
}

fn evaluate_mul(s: &Vec<char>, mut off: usize) -> (usize, i64) {
    while off < s.len() && s[off].is_whitespace() { off += 1 }
    let (mut off, mut res) = evaluate_add(s, off);
    while off < s.len() && s[off].is_whitespace() { off += 1 }
    while off < s.len() && s[off] == '*' {
        let (noff, sub) = evaluate_add(s, off + 1);
        off = noff;
        res *= sub;
        while off < s.len() && s[off].is_whitespace() { off += 1 }
    }
    return (off, res);
}

fn evaluate(s: &str) -> i64 {
    let (_, res) = evaluate_mul(&s.chars().collect(), 0);
    return res;
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

