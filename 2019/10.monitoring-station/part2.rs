
use std::io;
use std::io::prelude::*;
use std::collections::*;

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    return a;
}

fn reduce(frac: (i64, i64)) -> (i64, i64) {
    let div = gcd(frac.0.abs(), frac.1.abs());
    return (frac.0 / div, frac.1 / div);
}

fn count_visible(from: (i64, i64), all: &Vec<(i64, i64)>) -> i64 {
    let mut angles = HashSet::new();
    for p in all {
        if *p != from {
            angles.insert(reduce((p.0 - from.0, p.1 - from.1)));
        }
    }
    return angles.len() as i64;
}

fn destoyed_200th(from: (i64, i64), all: &Vec<(i64, i64)>) -> (i64, i64) {
    let mut dists = Vec::new();
    for p in all {
        if *p != from {
            dists.push((
                reduce((p.0 - from.0, p.1 - from.1)),
                (p.0 - from.0).abs() + (p.1 - from.1).abs(),
                *p
            ));
        }
    }
    let mut angles = Vec::new();
    dists.sort();
    let mut last = (0, 0);
    let mut count = 0;
    for i in 0..dists.len() {
        if last == dists[i].0 {
            count += 1;
        } else {
            count = 0;
            last = dists[i].0;
        }
        let angle = -f64::atan2(dists[i].0.1 as f64, dists[i].0.0 as f64);
        angles.push((count, angle, dists[i].2));
    }
    angles.sort_by(|a, b| a.partial_cmp(b).unwrap());
    return angles[199].2;
}

fn main() {
    let stdin = io::stdin();
    let mut points = Vec::new();
    for (i, l) in stdin.lock().lines().enumerate() {
        for (j, c) in l.unwrap().chars().enumerate() {
            if c == '#' {
                points.push((i as i64, j as i64));
            }
        }
    }
    let station = points.iter().max_by_key(|x| count_visible(**x, &points)).unwrap();
    let nth = destoyed_200th(*station, &points);
    println!("Result: {}", nth.1 * 100 + nth.0);
}

