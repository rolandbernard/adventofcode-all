
use std::io;
use std::io::prelude::*;

fn next_map(map: &Vec<Vec<char>>, neigh: &Vec<Vec<Vec<(usize, usize)>>>) -> (bool, Vec<Vec<char>>) {
    let mut change = false;
    let mut new = map.clone();
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            let mut occ = 0;
            for &p in &neigh[i][j] {
                if map[p.0][p.1] == '#' {
                    occ += 1;
                }
            }
            if map[i][j] == 'L' && occ == 0 {
                new[i][j] = '#';
                change = true;
            } else if map[i][j] == '#' && occ >= 5 {
                new[i][j] = 'L';
                change = true;
            }
        }
    }
    return (change, new);
}

fn main() {
    let stdin = io::stdin();
    let mut map = stdin.lock().lines()
        .map(|l| l.unwrap().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut neigh = vec![vec![Vec::new(); map[0].len()]; map.len()];
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            for di in 0..3 {
                for dj in 0..3 {
                    if di != 1 || dj != 1 {
                        let mut pos = ((i + di).wrapping_sub(1), (j + dj).wrapping_sub(1));
                        while pos.0 < map.len() && pos.1 < map[i].len() && map[pos.0][pos.1] == '.' {
                            pos = ((pos.0 + di).wrapping_sub(1), (pos.1 + dj).wrapping_sub(1));
                        }
                        if pos.0 < map.len() && pos.1 < map[i].len() && map[pos.0][pos.1] != '.' {
                            neigh[i][j].push(pos);
                        }
                    }
                }
            }
        }
    }
    loop {
        let (change, new) = next_map(&map, &neigh);
        map = new;
        if !change {
            break;
        }
    }
    println!("Result: {}", map.iter().map(|r| r.iter()).flatten().filter(|&&c| c == '#').count());
}

