
use std::io;
use std::io::prelude::*;
use std::collections::*;

fn main() {
    let stdin = io::stdin();
    let mut points = HashSet::<(i64, i64)>::new();
    for l in stdin.lock().lines() {
        let line = l.unwrap();
        let splt = line.split(", ").collect::<Vec<_>>();
        let fst = splt[0].split("=").collect::<Vec<_>>();
        let fst_val = fst[1].split("..").map(|x| x.parse().unwrap()).collect::<Vec<_>>();
        let snd = splt[1].split("=").collect::<Vec<_>>();
        let snd_val = snd[1].split("..").map(|x| x.parse().unwrap()).collect::<Vec<_>>();
        for i in if fst_val.len() == 1 { fst_val[0]..=fst_val[0] } else { fst_val[0]..=fst_val[1] } {
            for j in if snd_val.len() == 1 { snd_val[0]..=snd_val[0] } else { snd_val[0]..=snd_val[1] } {
                if fst[0] == "x" {
                    points.insert((i, j));
                } else {
                    points.insert((j, i));
                }
            }
        }
    }
    let top = points.iter().map(|x| x.1).min().unwrap();
    let deph = points.iter().map(|x| x.1).max().unwrap();
    let mut water = HashSet::<(i64, i64)>::new();
    let mut heads = HashSet::new();
    let mut fill = HashSet::new();
    heads.insert((500, 0));
    while heads.len() > 0 {
        let mut next_heads = HashSet::new();
        for mut h in heads {
            while !points.contains(&h) && h.1 <= deph {
                water.insert(h);
                h.1 += 1;
            }
            if h.1 <= deph {
                for d in 1.. {
                    let mut l = (h.0, h.1 - d);
                    while !points.contains(&l) && points.contains(&(l.0, l.1 + 1)) {
                        water.insert(l);
                        l.0 -= 1;
                    }
                    let mut r = (h.0, h.1 - d);
                    while !points.contains(&r) && points.contains(&(r.0, r.1 + 1)) {
                        water.insert(r);
                        r.0 += 1;
                    }
                    if !points.contains(&l) || !points.contains(&r) {
                        if !points.contains(&l) {
                            next_heads.insert(l);
                        }
                        if !points.contains(&r) {
                            next_heads.insert(r);
                        }
                        break;
                    } else {
                        for i in l.0 + 1..r.0 {
                            points.insert((i, l.1));
                            fill.insert((i, l.1));
                        }
                    }
                }
            }
        }
        heads = next_heads;
    }
    println!("Result: {}", fill.iter().filter(|x| x.1 >= top && x.1 <= deph).count());
}

