
use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let mut cups = stdin.lock().lines().next().unwrap().unwrap()
        .chars().map(|x| x.to_digit(10).unwrap() as usize - 1).collect::<Vec<_>>();
    let len = cups.len();
    let mut cur = 0;
    for _ in 0..100 {
        let cur_val = cups[cur];
        let pick = [
            cups[(cur + 1) % len],
            cups[(cur + 2) % len],
            cups[(cur + 3) % len]
        ];
        let mut search = (cur_val + len - 1) % len;
        let mut dest = cups.iter().position(|&x| x == search).unwrap();
        while dest == (cur + 1) % len || dest == (cur + 2) % len || dest == (cur + 3) % len {
            search = (search + len - 1) % len;
            dest = cups.iter().position(|&x| x == search).unwrap();
        }
        let mut i = (cur + 1) % len;
        while (i + 2) % len != dest {
            cups.swap(i, (i + 3) % len);
            i = (i + 1) % len;
        }
        for j in 0..3 {
            cups[i] = pick[j];
            i = (i + 1) % len;
        }
        cur = (cur + 1) % len;
    }
    let res = cups.iter().cycle()
        .skip_while(|&&x| x != 0).skip(1).take(len - 1)
        .map(|&x| char::from_digit(x as u32 + 1, 10).unwrap())
        .collect::<String>();
    println!("Result: {}", res);
}

