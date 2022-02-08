
use std::io;
use std::io::prelude::*;
use std::collections::*;

#[derive(Debug, Copy, Clone)]
enum Dir { Up, Down, Left, Right }

fn move_loc(from: (usize, usize), map: &Vec<Vec<char>>) -> Option<Dir> {
    let mut dist = vec![vec![usize::MAX; map[0].len()]; map.len()];
    let mut queue = VecDeque::new();
    dist[from.0][from.1] = 0;
    queue.push_back(((from.0 - 1, from.1), 1, Dir::Up));
    queue.push_back(((from.0, from.1 - 1), 1, Dir::Left));
    queue.push_back(((from.0, from.1 + 1), 1, Dir::Right));
    queue.push_back(((from.0 + 1, from.1), 1, Dir::Down));
    while !queue.is_empty() {
        let ((y, x), d, s) = queue.pop_front().unwrap();
        if map[y][x] == '.' {
            if dist[y][x] > d {
                dist[y][x] = d;
                queue.push_back(((y + 1, x), d + 1, s));
                queue.push_back(((y - 1, x), d + 1, s));
                queue.push_back(((y, x + 1), d + 1, s));
                queue.push_back(((y, x - 1), d + 1, s));
            }
        } else if map[y][x] != '#' && map[y][x] != map[from.0][from.1] {
            if d == 1 {
                return None;
            } else {
                return Some(s);
            }
        }
    }
    return None;
}

fn main() {
    let stdin = io::stdin();
    let mut map = stdin.lock().lines()
        .map(|x| x.unwrap().chars().collect())
        .collect::<Vec<Vec<_>>>();
    let mut ents = Vec::new();
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 'E' || map[i][j] == 'G' {
                ents.push(((i, j), 200, 3, map[i][j]));
            }
        }
    }
    let mut round = 0;
    'outer: loop {
        for e in 0..ents.len() {
            if ents[e].1 > 0 {
                if !ents.iter().any(|x| x.0 != ents[e].0 && x.1 > 0 && x.3 != ents[e].3) {
                    break 'outer;
                }
                if let Some(d) = move_loc(ents[e].0, &map) {
                    map[ents[e].0.0][ents[e].0.1] = '.';
                    match d {
                        Dir::Up => ents[e].0.0 -= 1,
                        Dir::Left => ents[e].0.1 -= 1,
                        Dir::Right => ents[e].0.1 += 1,
                        Dir::Down => ents[e].0.0 += 1,
                    }
                    map[ents[e].0.0][ents[e].0.1] = ents[e].3;
                }
                let mut g = usize::MAX;
                for j in 0..ents.len() {
                    if ents[j].1 > 0 && ents[j].3 != ents[e].3 {
                        if (ents[j].0.0 + 1, ents[j].0.1) == ents[e].0
                            || (ents[j].0.0, ents[j].0.1 + 1) == ents[e].0
                            || (ents[e].0.0 + 1, ents[e].0.1) == ents[j].0
                            || (ents[e].0.0, ents[e].0.1 + 1) == ents[j].0
                        {
                            if g == usize::MAX || ents[j].1 < ents[g].1 || (ents[j].1 == ents[g].1 && ents[j].0 < ents[g].0) {
                                g = j;
                            }
                        }
                    }
                }
                if g != usize::MAX {
                    ents[g].1 -= ents[e].2;
                    if ents[g].1 <= 0 {
                        map[ents[g].0.0][ents[g].0.1] = '.';
                    }
                }
            }
        }
        ents.sort();
        round += 1;
    }
    println!("Result: {}", round * ents.iter().map(|x| x.1).filter(|x| *x > 0).sum::<i64>());
}

