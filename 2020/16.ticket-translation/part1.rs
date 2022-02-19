
use std::io;
use std::io::prelude::*;

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
    let mut sum = 0;
    for o in others {
        for v in o {
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
                sum += v;
            }
        }
    }
    println!("Result: {}", sum);
}

