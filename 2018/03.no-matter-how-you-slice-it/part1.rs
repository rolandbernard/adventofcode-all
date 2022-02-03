
use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let mut claims = vec![vec![0; 1000]; 1000];
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        let splt = l.split(" ").collect::<Vec<&str>>();
        let pos = (&splt[2][0..splt[2].len() - 1]).split(",")
            .map(|x| x.parse().unwrap()).collect::<Vec<usize>>();
        let size = splt[3].split("x").map(|x| x.parse().unwrap())
            .collect::<Vec<usize>>();
        for i in pos[0]..pos[0] + size[0] {
            for j in pos[1]..pos[1] + size[1] {
                claims[i][j] += 1;
            }
        }
    }
    let mut count = 0;
    for row in claims {
        for x in row {
            if x > 1 {
                count += 1;
            }
        }
    }
    println!("Result: {}", count);
}
