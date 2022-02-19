
use std::io;
use std::io::prelude::*;

#[derive(Debug, Clone, Copy)]
enum Token {
    Term(char),
    NonTerm(usize),
}

fn try_rule(
    d: usize, rule: &Vec<Token>, s: &[char], from: usize, to: usize, 
    rules: &Vec<Vec<Vec<Token>>>, cache: &mut Vec<Vec<Vec<Option<bool>>>>
) -> bool {
    if d == rule.len() - 1 {
        return matches(s, from, to, rule[d], rules, cache);
    } else {
        for j in from..to {
            if matches(s, from, j, rule[d], rules, cache) && try_rule(d + 1, rule, s, j, to, rules, cache) {
                return true;
            }
        }
        return false;
    }
}

fn matches(
    s: &[char], from: usize, to: usize, rule: Token,
    rules: &Vec<Vec<Vec<Token>>>, cache: &mut Vec<Vec<Vec<Option<bool>>>>
) -> bool {
    match rule {
        Token::Term(c) => {
            return from + 1 == to && s[from] == c;
        },
        Token::NonTerm(r) => {
            if from == to {
                return false;
            } else if let Some(p) = cache[r][from][to] {
                return p;
            } else  {
                let mut pos = false;
                for rep in &rules[r] {
                    if try_rule(0, rep, s, from, to, rules, cache) {
                        pos = true;
                        break;
                    }
                }
                cache[r][from][to] = Some(pos);
                return pos;
            }
        },
    }
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
    rules[8] = vec![vec![Token::NonTerm(42)], vec![Token::NonTerm(42), Token::NonTerm(8)]];
    rules[11] = vec![vec![Token::NonTerm(42), Token::NonTerm(31)], vec![Token::NonTerm(42), Token::NonTerm(11), Token::NonTerm(31)]];
    i += 1;
    let mut count = 0;
    while i < lines.len() {
        let mut cache = vec![vec![vec![None; lines.len() + 1]; lines.len() + 1]; rules.len()];
        let vec = lines[i].chars().collect::<Vec<_>>();
        if matches(&vec[..], 0, vec.len(), Token::NonTerm(0), &rules, &mut cache) {
            count += 1;
        }
        i += 1;
    }
    println!("Result: {}", count);
}

