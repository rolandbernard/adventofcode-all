
use std::io;
use std::io::prelude::*;

fn count_neigh(map: &Vec<Vec<char>>) -> Vec<Vec<(usize, usize)>> {
    let mut ret = vec![vec![(0, 0); map[0].len()]; map.len()];
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            for di in 0..3 {
                for dj in 0..3 {
                    if (di != 1 || dj != 1)
                        && i + di >= 1 && j + dj >= 1
                        && i + di <= map.len() && j + dj <= map[i].len()
                    {
                        if map[i + di - 1][j + dj - 1] == '|' {
                            ret[i][j].0 += 1;
                        } else if map[i + di - 1][j + dj - 1] == '#' {
                            ret[i][j].1 += 1;
                        }
                    }
                }
            }
        }
    }
    return ret;
}

fn main() {
    let stdin = io::stdin();
    let mut map = stdin.lock().lines()
        .map(|x| x.unwrap().chars().collect())
        .collect::<Vec<Vec<_>>>();
    for _ in 0..10 {
        let count = count_neigh(&map);
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

