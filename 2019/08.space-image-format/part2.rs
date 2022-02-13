
use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let img = stdin.lock().lines().next().unwrap().unwrap().bytes()
        .map(|x| (x - '0' as u8) as usize).collect::<Vec<_>>();
    let layers = img.len() / (25 * 6);
    let mut dec = [[0; 25]; 6];
    for i in (0..layers).rev() {
        for y in 0..6 {
            for x in 0..25 {
                let data = img[x + 25 * (y + 6 * i)];
                if data != 2 {
                    dec[y][x] = data;
                }
            }
        }
    }
    println!("Result:");
    for y in 0..6 {
        for x in 0..25 {
            print!("{}", if dec[y][x] == 0 {' '} else {'#'});
        }
        println!();
    }
}

