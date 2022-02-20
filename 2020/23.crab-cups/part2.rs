
use std::io;
use std::io::prelude::*;

const N: usize = 1_000_000;
const M: usize = 10_000_000;

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();
    let mut next = vec![usize::MAX; N];
    let mut last = N - 1;
    for c in input.chars() {
        let v = (c.to_digit(10).unwrap() - 1) as usize;
        next[last] = v;
        last = v;
    }
    for i in input.len()..N {
        next[last] = i;
        last = i;
    }
    let mut cur = input.chars().next().unwrap().to_digit(10).unwrap() as usize - 1;
    next[last] = cur;
    for _ in 0..M {
        let mut pick = [next[cur], 0, 0, 0];
        for i in 1..4 {
            pick[i] = next[pick[i - 1]];
        }
        let mut dest = (cur + N - 1) % N;
        while dest == pick[0] || dest == pick[1] || dest == pick[2] {
            dest = (dest + N - 1) % N;
        }
        next[pick[2]] = next[dest];
        next[dest] = pick[0];
        next[cur] = pick[3];
        cur = next[cur];
    }
    println!("Result: {}", (next[0] + 1) * (next[next[0]] + 1));
}

