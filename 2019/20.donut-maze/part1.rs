
use std::io;
use std::io::prelude::*;
use std::collections::*;

fn main() {
    let stdin = io::stdin();
    let map = stdin.lock().lines()
        .map(|l| l.unwrap().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut labels = HashMap::new();
    for i in 2..map.len()-2 {
        for j in 2..map[i].len()-2 {
            if map[i][j] == '.' {
                let mut key = None;
                if map[i][j - 1].is_alphabetic() && map[i][j - 2].is_alphabetic() {
                    key = Some((map[i][j - 2], map[i][j - 1]));
                } else if map[i][j + 1].is_alphabetic() && map[i][j + 2].is_alphabetic() {
                    key = Some((map[i][j + 1], map[i][j + 2]));
                } else if map[i - 1][j].is_alphabetic() && map[i - 2][j].is_alphabetic() {
                    key = Some((map[i - 2][j], map[i - 1][j]));
                } else if map[i + 1][j].is_alphabetic() && map[i + 2][j].is_alphabetic() {
                    key = Some((map[i + 1][j], map[i + 2][j]));
                }
                if let Some(label) = key {
                    if !labels.contains_key(&label) {
                        labels.insert(label, vec![(i, j)]);
                    } else {
                        labels.get_mut(&label).unwrap().push((i, j));
                    }
                }
            }
        }
    }
    let mut portals = HashMap::new();
    for (_, v) in &labels {
        if v.len() == 2 {
            for i in 0..2 {
                portals.insert(v[i], v[1 - i]);
                portals.insert(v[1 - i], v[i]);
            }
        }
    }
    let mut dists = vec![vec![i64::MAX; map[0].len()]; map.len()];
    let mut queue = VecDeque::new();
    let start = labels[&('A', 'A')][0];
    let end = labels[&('Z', 'Z')][0];
    dists[start.0][start.1] = 0;
    queue.push_back((0, start));
    while let Some((d, pos)) = queue.pop_front() {
        if pos == end {
            break;
        } else {
            for del in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let next = ((pos.0 as i64 + del.0) as usize, (pos.1 as i64 + del.1) as usize);
                if map[next.0][next.1] == '.' && dists[next.0][next.1] > d + 1 {
                    dists[next.0][next.1] = d + 1;
                    queue.push_back((d + 1, next));
                }
            }
            if portals.contains_key(&pos) {
                let next = portals[&pos];
                if dists[next.0][next.1] > d + 1 {
                    dists[next.0][next.1] = d + 1;
                    queue.push_back((d + 1, next));
                }
            }
        }
    }
    println!("Result: {}", dists[end.0][end.1]);
}

