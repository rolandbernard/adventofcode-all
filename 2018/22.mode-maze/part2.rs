
use std::io;
use std::io::prelude::*;
use std::collections::*;
use std::cmp::*;

struct Map {
    depth: i64,
    target: (i64, i64),
    map: HashMap<(i64, i64), i64>,
}

impl Map {
    fn erosion_at(&mut self, pos: (i64, i64)) -> i64 {
        if pos == (0, 0) || pos == self.target {
            return self.depth % 20183;
        } else if pos.1 == 0 {
            return (pos.0 * 16807 + self.depth) % 20183
        } else if pos.0 == 0 {
            return (pos.1 * 48271 + self.depth) % 20183
        } else {
            if !self.map.contains_key(&pos) {
                let index = self.erosion_at((pos.0 - 1, pos.1)) * self.erosion_at((pos.0, pos.1 - 1));
                self.map.insert(pos, (index + self.depth) % 20183);
            }
            return self.map[&pos];
        }
    }

    fn type_at(&mut self, pos: (i64, i64)) -> i64 {
        return self.erosion_at(pos) % 3;
    }
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines()
        .map(|x| x.unwrap())
        .collect::<Vec<_>>();
    let depth = lines[0].split(" ").skip(1).next().unwrap()
        .parse::<i64>().unwrap();
    let target = lines[1].split(" ").skip(1).next().unwrap().split(",")
        .map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>();
    let mut map = Map { depth, target: (target[0], target[1]), map: HashMap::new() };
    let mut dists = HashMap::new();
    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0, (0, 0), 0)));
    while let Some(Reverse((d, pos, eq))) = queue.pop() {
        if !dists.contains_key(&(pos, eq)) || dists[&(pos, eq)] > d {
            dists.insert((pos, eq), d);
            if pos == map.target && eq == 0 {
                break;
            }
            let (x, y) = pos;
            let t = map.type_at(pos);
            if eq == t {
                queue.push(Reverse((d + 7, pos, (t + 1) % 3)));
            } else {
                queue.push(Reverse((d + 7, pos, t)));
            }
            for n in [(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)] {
                if n.0 >= 0 && n.1 >= 0 {
                    let nt = map.type_at(n);
                    if nt == eq || (nt + 1) % 3 == eq {
                        queue.push(Reverse((d + 1, n, eq)));
                    }
                }
            }
        }
    }
    println!("Result: {}", dists[&(map.target, 0)]);
}

