
use std::io;
use std::io::prelude::*;
use std::collections::*;
use std::cmp::*;

fn possible_keys<'a>(
    from: (i64, i64), key: u32, map: &Vec<Vec<char>>, cache: &'a mut HashMap<((i64, i64), u32), Vec<(i64, u32, (i64, i64))>>
) -> &'a Vec<(i64, u32, (i64, i64))> {
    if !cache.contains_key(&(from, key)) {
        let mut dists = vec![vec![i64::MAX; map[0].len()]; map.len()];
        let mut queue = VecDeque::new();
        let mut keys = Vec::new();
        dists[from.0 as usize][from.1 as usize] = 0;
        queue.push_back((0, from));
        while let Some((d, pos)) = queue.pop_front() {
            if dists[pos.0 as usize][pos.1 as usize] == d {
                if map[pos.0 as usize][pos.1 as usize] as u8 >= 'a' as u8
                    && map[pos.0 as usize][pos.1 as usize] as u8 <= 'z' as u8
                {
                    keys.push((d, (map[pos.0 as usize][pos.1 as usize] as u8 - 'a' as u8) as u32, pos));
                }
                for del in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                    let next = (pos.0 + del.0, pos.1 + del.1);
                    let c = map[next.0 as usize][next.1 as usize];
                    if c != '#' {
                        if c as u8 >= 'A' as u8 && c as u8 <= 'Z' as u8 {
                            let k = (c as u8 - 'A' as u8) as u32;
                            if (key & (1 << k)) == 0 {
                                continue;
                            }
                        }
                        if dists[next.0 as usize][next.1 as usize] > d + 1 {
                            dists[next.0 as usize][next.1 as usize] = d + 1;
                            queue.push_back((d + 1, next));
                        }
                    }
                }
            }
        }
        cache.insert((from, key), keys);
    }
    return &cache[&(from, key)];
}

fn main() {
    let stdin = io::stdin();
    let mut map = stdin.lock().lines()
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
    for i in 0..3 {
        for j in 0..3 {
            if i == 1 || j == 1 {
                map[enter.0 as usize + i - 1][enter.1 as usize + j - 1] = '#';
            } else {
                map[enter.0 as usize + i - 1][enter.1 as usize + j - 1] = '@';
            }
        }
    }
    let start = [
        (enter.0 - 1, enter.1 - 1),
        (enter.0 - 1, enter.1 + 1),
        (enter.0 + 1, enter.1 - 1),
        (enter.0 + 1, enter.1 + 1),
    ];
    let mut cache = HashMap::new();
    let mut dists = HashMap::new();
    let mut queue = BinaryHeap::new();
    dists.insert((start.clone(), 0), 0);
    queue.push(Reverse((0, (start.clone(), 0))));
    let mut res = 0;
    while let Some(Reverse((d, (pos, keys)))) = queue.pop() {
        if keys == (1 << key_count) - 1 {
            res = d;
            break;
        } else {
            for r in 0..4 {
                for &(dd, k, np) in possible_keys(pos[r], keys, &map, &mut cache) {
                    let mut next = (pos, keys | (1 << k));
                    next.0[r] = np;
                    if !dists.contains_key(&next) || dists[&next] > d + dd {
                        dists.insert(next, d + dd);
                        queue.push(Reverse((d + dd, next)));
                    }
                }
            }
        }
    }
    println!("Result: {}", res);
}

