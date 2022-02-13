
use std::io;
use std::io::prelude::*;

type Point = (i64, i64);
type Line = (Point, Point);

fn intersection(l1: Line, l2: Line) -> Option<i64> {
    if l1.0.0 == l1.1.0 {
        if l2.0.0 == l2.1.0 {
            if l1.0.0 == l2.0.0 && l1.0.1.max(l1.1.1) >= l2.0.1.min(l2.1.1) && l1.0.1.min(l1.1.1) <= l2.0.1.max(l2.1.1) {
                let top = (l1.0.1.min(l1.1.1)).max(l2.0.1.min(l2.1.1));
                let bot = (l1.0.1.max(l1.1.1)).min(l2.0.1.max(l2.1.1));
                if top >= 0 && bot <= 0 {
                    return Some(l1.0.0.abs());
                } else {
                    return Some(l1.0.0.abs() + top.abs().min(bot.abs()));
                }
            } else {
                return None;
            }
        } else {
            if l1.0.1.min(l1.1.1) <= l2.0.1 && l1.0.1.max(l1.1.1) >= l2.0.1
                && l2.0.0.min(l2.1.0) <= l1.0.0 && l2.0.0.max(l2.1.0) >= l1.0.0
            {
                return Some(l2.0.1.abs() + l1.0.0.abs());
            } else {
                return None;
            }
        }
    } else {
        return intersection(((l1.0.1, l1.0.0), (l1.1.1, l1.1.0)), ((l2.0.1, l2.0.0), (l2.1.1, l2.1.0)));
    }
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines()
        .map(|x| x.unwrap().split(",")
            .map(|x| (x.chars().next().unwrap(), x[1..].parse::<i64>().unwrap()))
            .collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut segs = Vec::new();
    for line in lines {
        let mut pos = (0, 0);
        let mut seg = Vec::new();
        for i in line {
            let start = pos;
            match i {
                ('U', a) => pos.1 -= a,
                ('D', a) => pos.1 += a,
                ('L', a) => pos.0 -= a,
                ('R', a) => pos.0 += a,
                _ => {},
            }
            seg.push((start, pos));
        }
        segs.push(seg);
    }
    let mut min = i64::MAX;
    for s in 0..segs[0].len() {
        for o in 0..segs[1].len() {
            if let Some(d) = intersection(segs[0][s], segs[1][o]) {
                if d != 0 {
                    min = min.min(d);
                }
            }
        }
    }
    println!("Result: {}", min);
}

