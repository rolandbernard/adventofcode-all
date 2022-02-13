
use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let img = stdin.lock().lines().next().unwrap().unwrap().bytes()
        .map(|x| (x - '0' as u8) as usize).collect::<Vec<_>>();
    let layers = img.len() / (25 * 6);
    let mut min = (usize::MAX, 0, 0);
    for i in 0..layers {
        let mut count = [0; 10];
        for j in 0..25 * 6 {
            count[img[i * (25 * 6) + j]] += 1;
        }
        if min.0 > count[0] {
            min = (count[0], count[1], count[2]);
        }
    }
    println!("Result: {}", min.1 * min.2);
}

