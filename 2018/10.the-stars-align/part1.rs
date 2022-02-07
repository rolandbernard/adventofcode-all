
use std::io;
use std::io::prelude::*;

fn bounds(points: &Vec<(i64, i64, i64, i64)>, step: i64) -> (i64, i64, i64, i64) {
    let mut res = (i64::MAX, i64::MAX, i64::MIN, i64::MIN);
    for p in points {
        res.0 = res.0.min(p.0 + p.2 * step);
        res.1 = res.1.min(p.1 + p.3 * step);
        res.2 = res.2.max(p.0 + p.2 * step);
        res.3 = res.3.max(p.1 + p.3 * step);
    }
    return res;
}

fn area(bound: (i64, i64, i64, i64)) -> i64 {
    return (bound.2 - bound.0 + 1) * (bound.3 - bound.1 + 1);
}

fn main() {
    let stdin = io::stdin();
    let points: Vec<(i64, i64, i64, i64)> = stdin.lock().lines()
        .map(|x| x.unwrap()
            .split(|c| c == '<' || c == '>' || c == ',')
            .map(|s| s.trim().to_owned()).collect())
        .map(|x: Vec<String>| (
            x[1].parse().unwrap(), x[2].parse().unwrap(),
            x[4].parse().unwrap(), x[5].parse().unwrap()))
        .collect();
    let mut res = 0;
    let mut last = i64::MAX;
    for i in 1.. {
        let bound = bounds(&points, i);
        let area = area(bound);
        if area > last {
            res = i - 1;
            break;
        }
        last = area;
    }
    let bound = bounds(&points, res);
    let mut map = vec![vec!['.'; (bound.2 - bound.0 + 1) as usize]; (bound.3 - bound.1 + 1) as usize];
    for p in &points {
        map[(p.1 - bound.1 + res * p.3) as usize][(p.0 - bound.0 + res * p.2) as usize] = '#';
    }
    println!("Result:");
    for row in &map {
        for &x in row {
            print!("{}", x);
        }
        println!();
    }
}
