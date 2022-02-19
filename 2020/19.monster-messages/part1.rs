
use std::io;
use std::io::prelude::*;

#[derive(Debug, Clone, Copy)]
enum Token {
    Term(char),
    NonTerm(usize),
}

fn matches(s: &[char], off: usize, rule: usize, rules: &Vec<Vec<Vec<Token>>>) -> Option<usize> {
    'pos: for pos in &rules[rule] {
        let mut of = off;
        for &g in pos {
            if let Token::Term(c) = g {
                if s[of] != c {
                    continue 'pos;
                }
                of += 1;
            } else if let Token::NonTerm(typ) = g {
                if let Some(nof) = matches(s, of, typ, rules) {
                    of = nof;
                } else {
                    continue 'pos;
                }
            }
        }
        return Some(of);
    }
    return None;
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines()
        .map(|l| l.unwrap()).collect::<Vec<_>>();
    let mut i = 0;
    let mut rules = Vec::new();
    while lines[i] != "" {
        let splt = lines[i].split(": ").collect::<Vec<_>>();
        let idx = splt[0].parse::<usize>().unwrap();
        if idx >= rules.len() {
            rules.resize_with(idx + 1, || Vec::new());
        }
        if splt[1].chars().next().unwrap() == '"' {
            rules[idx].push(vec![
                Token::Term(splt[1].chars().skip(1).next().unwrap())
            ]);
        } else {
            for pos in splt[1].split(" | ") {
                rules[idx].push(
                    pos.split(" ")
                        .map(|x| x.parse::<usize>().unwrap())
                        .map(|x| Token::NonTerm(x))
                        .collect::<Vec<_>>()
                );
            }
        }
        i += 1;
    }
    i += 1;
    let mut count = 0;
    while i < lines.len() {
        let vec = lines[i].chars().collect::<Vec<_>>();
        if matches(&vec[..], 0, 0, &rules) == Some(vec.len()) {
            count += 1;
        }
        i += 1;
    }
    println!("Result: {}", count);
}

