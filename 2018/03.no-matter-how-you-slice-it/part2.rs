
use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let mut claims = Vec::new();
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        let splt = l.split(" ").collect::<Vec<&str>>();
        let pos = splt[2][0..splt[2].len() - 1].split(",")
            .map(|x| x.parse().unwrap()).collect::<Vec<usize>>();
        let size = splt[3].split("x").map(|x| x.parse().unwrap())
            .collect::<Vec<usize>>();
        claims.push((
            splt[0][1..splt[0].len()].parse::<usize>().unwrap(),
            (pos[0], pos[1]), (size[0], size[1])
        ));
    }
    let mut map = vec![vec![0; 1000]; 1000];
    for cl in &claims {
        for i in cl.1.0..cl.1.0 + cl.2.0 {
            for j in cl.1.1..cl.1.1 + cl.2.1 {
                map[i][j] += 1;
            }
        }
    }
    let uniq = 'ret: loop {
        'outer: for cl in &claims {
            for i in cl.1.0..cl.1.0 + cl.2.0 {
                for j in cl.1.1..cl.1.1 + cl.2.1 {
                    if map[i][j] > 1 {
                        continue 'outer;
                    }
                }
            }
            break 'ret cl.0;
        }
    };
    println!("Result: {}", uniq);
}
