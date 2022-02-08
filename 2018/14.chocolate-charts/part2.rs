
use std::io;
use std::io::prelude::*;
use std::cmp::*;

fn ends_with<T: PartialEq>(vec: &Vec<T>, pat: &Vec<T>) -> bool {
    if vec.len() < pat.len() {
        return false;
    } else {
        for i in 0..pat.len() {
            if vec[vec.len() - pat.len() + i] != pat[i] {
                return false;
            }
        }
        return true;
    }
}

fn main() {
    let stdin = io::stdin();
    let pat = stdin.lock().lines().next().unwrap().unwrap().bytes()
        .map(|x| x - '0' as u8).collect::<Vec<_>>();
    let mut pos = (0, 1);
    let mut rec = vec![3, 7];
    loop {
        let sum = rec[pos.0] + rec[pos.1];
        if sum >= 10 {
            rec.push(sum / 10);
            if ends_with(&rec, &pat) {
                break;
            }
        }
        rec.push(sum % 10);
        if ends_with(&rec, &pat) {
            break;
        }
        pos.0 = (pos.0 + 1 + rec[pos.0] as usize) % rec.len();
        pos.1 = (pos.1 + 1 + rec[pos.1] as usize) % rec.len();
    }
    println!("Result: {}", rec.len() - pat.len());
}

