
use std::io;
use std::io::prelude::*;
use std::collections::*;

const N: usize = 1000000000;

fn count_neigh(map: &Vec<Vec<char>>, counts: &mut Vec<Vec<(usize, usize)>>) {
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            counts[i][j] = (0, 0);
            for di in 0..3 {
                for dj in 0..3 {
                    if (di != 1 || dj != 1)
                        && i + di >= 1 && j + dj >= 1
                        && i + di <= map.len() && j + dj <= map[i].len()
                    {
                        if map[i + di - 1][j + dj - 1] == '|' {
                            counts[i][j].0 += 1;
                        } else if map[i + di - 1][j + dj - 1] == '#' {
                            counts[i][j].1 += 1;
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut map = stdin.lock().lines()
        .map(|x| x.unwrap().chars().collect())
        .collect::<Vec<Vec<_>>>();
    let mut seen = HashMap::new();
    let mut state = Vec::new();
    let mut count = vec![vec![(0, 0); map[0].len()]; map.len()];
    for i in 0..N {
        if !seen.contains_key(&map) {
            state.push(map.clone());
            seen.insert(map.clone(), i);
            count_neigh(&map, &mut count);
            for i in 0..map.len() {
                for j in 0..map[i].len() {
                    if map[i][j] == '.' {
                        if count[i][j].0 >= 3 {
                            map[i][j] = '|';
                        }
                    } else if map[i][j] == '|' {
                        if count[i][j].1 >= 3 {
                            map[i][j] = '#';
                        }
                    } else if map[i][j] == '#' {
                        if count[i][j].0 == 0 || count[i][j].1 == 0 {
                            map[i][j] = '.';
                        }
                    }
                }
            }
        } else {
            let p = i - seen[&map];
            let off = (N - i) % p;
            map = state[seen[&map] + off].clone();
            break;
        }
    }
    let mut count = (0, 0);
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == '|' {
                count.0 += 1;
            } else if map[i][j] == '#' {
                count.1 += 1;
            }
        }
    }
    println!("Result: {}", count.0 * count.1);
}

