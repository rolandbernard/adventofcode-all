
use std::io;
use std::io::prelude::*;
use std::collections::*;

fn main() {
    let stdin = io::stdin();
    let map = stdin.lock().lines()
        .map(|l| l.unwrap().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut key_count = 0;
    let mut enter = (0, 0);
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            let c = map[i][j];
            if c == '@' {
                enter = (i as i64, j as i64)
            } else if c as u8 >= 'a' as u8 && c as u8 <= 'z' as u8 {
                key_count += 1;
            }
        }
    }
    let mut dists = HashMap::new();
    let mut queue = VecDeque::new();
    dists.insert((enter, 0), 0);
    queue.push_back((0, (enter, 0)));
    let mut res = 0;
    while let Some((d, (pos, keys))) = queue.pop_front() {
        if keys == (1 << key_count) - 1 {
            res = d;
            break;
        } else {
            for del in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let mut next = ((pos.0 + del.0, pos.1 + del.1), keys);
                let c = map[next.0.0 as usize][next.0.1 as usize];
                if c != '#' {
                    if c as u8 >= 'a' as u8 && c as u8 <= 'z' as u8 {
                        let k = (c as u8 - 'A' as u8) as u32;
                        next.1 |= 1 << k;
                    } else if c as u8 >= 'A' as u8 && c as u8 <= 'Z' as u8 {
                        let k = (c as u8 - 'A' as u8) as u32;
                        if (next.1 & (1 << k)) == 0 {
                            continue;
                        }
                    }
                    if !dists.contains_key(&next) || dists[&next] > d + 1 {
                        dists.insert(next, d + 1);
                        queue.push_back((d + 1, next));
                    }
                }
            }
        }
    }
    println!("Result: {}", res);
}

