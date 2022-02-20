
use std::io;
use std::io::prelude::*;

const MOD: usize = 20201227;

fn pow_mod(mut x: usize, mut e: usize, m: usize) -> usize {
    let mut y = 1;
    while e > 1 {
        if e % 2 != 0 {
            y = y * x % m;
        }
        x = x * x % m;
        e /= 2;
    }
    return x * y % m;
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|x| x.unwrap().parse::<usize>().unwrap());
    let pk1 = lines.next().unwrap();
    let pk2 = lines.next().unwrap();
    let mut public = 1;
    let mut private = 0;
    loop {
        if public == pk1 || public == pk2 {
            public = if public == pk1 { pk2 } else { pk1 };
            break;
        }
        public = (public * 7) % MOD;
        private += 1;
    }
    println!("Result: {}", pow_mod(public, private, MOD));
}

