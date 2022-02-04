
use std::io;
use std::io::prelude::*;

fn get_type(byte: u8) -> (bool, u8) {
    if byte >= 'a' as u8 && byte <= 'z' as u8 {
        return (false, byte - 'a' as u8);
    } else {
        return (true, byte - 'A' as u8);
    }
}

fn main() {
    let stdin = io::stdin();
    let mut poly = stdin.lock().lines().next().unwrap().unwrap().into_bytes();
    let mut removed = 0;
    let mut first = 0;
    let mut second = 1;
    while second < poly.len() {
        let (p1, t1) = get_type(poly[first]);
        let (p2, t2) = get_type(poly[second]);
        if t1 == t2 && p2 != p1 {
            removed += 2;
            if first == 0 {
                poly[first] = poly[second + 1];
                second += 2;
            } else {
                first -= 1;
                second += 1;
            }
        } else {
            first += 1;
            poly[first] = poly[second];
            second += 1;
        }
    }
    println!("Result: {}", poly.len() - removed);
}

