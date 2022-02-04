
use std::io;
use std::io::prelude::*;

fn get_type(byte: u8) -> (bool, u8) {
    if byte >= 'a' as u8 && byte <= 'z' as u8 {
        return (false, byte - 'a' as u8);
    } else {
        return (true, byte - 'A' as u8);
    }
}

fn reduce_all(poly: &mut Vec<u8>) {
    let mut first = 0;
    let mut second = 1;
    while second < poly.len() {
        let (p1, t1) = get_type(poly[first]);
        let (p2, t2) = get_type(poly[second]);
        if t1 == t2 && p2 != p1 {
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
    poly.resize(first + 1, 0);
}

fn remove_type(poly: &mut Vec<u8>, typ: u8) {
    let mut i = 0;
    let mut j = 0;
    while j < poly.len() {
        let (_, t) = get_type(poly[j]);
        if t != typ {
            poly[i] = poly[j];
            i += 1;
        }
        j += 1;
    }
    poly.resize(i, 0);
}

fn main() {
    let stdin = io::stdin();
    let original = stdin.lock().lines().next().unwrap().unwrap().into_bytes();
    let mut min = original.len();
    for c in 0..26 {
        let mut rem = original.to_owned();
        remove_type(&mut rem, c);
        reduce_all(&mut rem);
        if rem.len() < min {
            min = rem.len();
        }
    }
    println!("Result: {}", min);
}

