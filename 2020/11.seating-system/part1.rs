
use std::io;
use std::io::prelude::*;

fn next_map(map: &Vec<Vec<char>>) -> (bool, Vec<Vec<char>>) {
    let mut change = false;
    let mut new = map.clone();
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            let mut occ = 0;
            for di in 0..3 {
                for dj in 0..3 {
                    if (di != 1 || dj != 1)
                        && i + di > 0 && j + dj > 0
                        && i + di < map.len() + 1 && j + dj < map[i].len() + 1
                        && map[i + di - 1][j + dj - 1] == '#'
                    {
                        occ += 1;
                    }
                }
            }
            if map[i][j] == 'L' && occ == 0 {
                new[i][j] = '#';
                change = true;
            } else if map[i][j] == '#' && occ >= 4 {
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
    loop {
        let (change, new) = next_map(&map);
        map = new;
        if !change {
            break;
        }
    }
    println!("Result: {}", map.iter().map(|r| r.iter()).flatten().filter(|&&c| c == '#').count());
}

