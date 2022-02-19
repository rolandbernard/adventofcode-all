
use std::io;
use std::io::prelude::*;
use std::collections::*;

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(|x| x.unwrap()).collect::<Vec<_>>();
    let mut fields = Vec::new();
    let mut i = 0;
    while lines[i] != "" {
        let splt = lines[i].split(": ").collect::<Vec<_>>();
        let ranges = splt[1].split(" or ")
            .map(|x| x.split("-")
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>())
            .collect::<Vec<_>>();
        fields.push((splt[0], ranges));
        i += 1;
    }
    let mine = lines[i + 2].split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    i += 5;
    let mut others = Vec::new();
    while i < lines.len() {
        others.push(
            lines[i].split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        );
        i += 1;
    }
    let mut pos = vec![(0..mine.len()).collect::<HashSet<_>>(); fields.len()];
    for o in others {
        let mut all_valid = true;
        for &v in &o {
            let mut valid = false;
            'outer: for (_, f) in &fields {
                for r in f {
                    if v >= r[0] && v <= r[1] {
                        valid = true;
                        break 'outer;
                    }
                }
            }
            if !valid {
                all_valid = false;
                break;
            }
        }
        if all_valid {
            for (j, &v) in o.iter().enumerate() {
                for (i, (_, f)) in fields.iter().enumerate() {
                    let mut valid = false;
                    for r in f {
                        if v >= r[0] && v <= r[1] {
                            valid = true;
                            break;
                        }
                    }
                    if !valid {
                        pos[i].remove(&j);
                    }
                }
            }
        }
    }
    let mut known = vec![None; pos.len()];
    loop {
        let mut change = false;
        for i in 0..pos.len() {
            if pos[i].len() == 1 {
                let v = *pos[i].iter().next().unwrap();
                for j in 0..pos.len() {
                    pos[j].remove(&v);
                }
                known[i] = Some(v);
                change = true;
            }
        }
        if !change {
            break;
        }
    }
    let mut res = 1;
    for (i, (n, _)) in fields.iter().enumerate() {
        if n.starts_with("departure") {
            res *= mine[known[i].unwrap()];
        }
    }
    println!("Result: {}", res);
}

