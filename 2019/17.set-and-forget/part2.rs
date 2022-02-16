
use std::io;
use std::io::prelude::*;

#[path ="../intcode/intcode.rs"]
mod intcode;

#[derive(PartialEq, Copy, Clone)]
enum Move {
    Left(i64),
    Right(i64),
}

fn get_map(mut mem: intcode::IntMem) -> Vec<Vec<i64>> {
    let mut cpu = intcode::IntState::new();
    let mut out = Vec::new();
    out.push(Vec::new());
    while !cpu.halt {
        intcode::run_instr(&mut mem, &mut cpu, || 0, |x| {
            if x == 10 {
                out.push(Vec::new());
            } else {
                let last = out.len() - 1;
                out[last].push(match x as u8 as char {
                    '#' => 1,
                    '^' => 2,
                    'v' => 3,
                    '>' => 4,
                    '<' => 5,
                    _ => 0,
                });
            }
        });
    }
    while out.last().unwrap().len() == 0 {
        out.pop();
    }
    return out;
}

fn path_string(path: &Vec<Move>) -> String {
    return path.iter().map(|x| match x {
        Move::Left(x) => "L,".to_owned() + &x.to_string() + ",",
        Move::Right(x) => "R,".to_owned() + &x.to_string() + ",",
    }).collect::<String>().trim_end_matches(|c| c == ',').to_owned();
}

fn call_string(calls: &Vec<i64>) -> String {
    return calls.iter()
        .map(|&x| ((x as u8 + 'A' as u8) as char).to_string() + ",")
        .collect::<String>().trim_end_matches(|c| c == ',').to_owned();
}

fn notify(mut mem: intcode::IntMem, main: &Vec<i64>, subs: &[Vec<Move>; 3]) -> i64 {
    let mut cpu = intcode::IntState::new();
    let string = [
        call_string(main), path_string(&subs[0]), path_string(&subs[1]), path_string(&subs[2]), "n\n".to_owned()
    ].join("\n");
    mem[0] = 2;
    let mut input = string.chars();
    let mut res = 0;
    while !cpu.halt {
        intcode::run_instr(&mut mem, &mut cpu, || input.next().unwrap() as i64, |x| res = x);
    }
    return res;
}

fn try_size(path: &Vec<Move>, s: [usize; 3]) -> Option<(Vec<i64>, [Vec<Move>; 3])> {
    let mut main = Vec::new();
    let mut subs = [Vec::new(), Vec::new(), Vec::new()];
    let mut i = 0;
    let mut j = 0;
    while i < path.len() {
        let mut kind = j;
        for k in 0..j {
            if i + subs[k].len() <= path.len() {
                let mut valid = true;
                for p in 0..subs[k].len() {
                    if path[i + p] != subs[k][p] {
                        valid = false;
                        break;
                    }
                }
                if valid {
                    kind = k;
                }
            }
        }
        if kind == j {
            if j < 3 && i + s[j] <= path.len() {
                for p in 0..s[j] {
                    subs[j].push(path[i + p]);
                }
                j += 1;
            } else {
                break;
            }
        }
        i += subs[kind].len();
        main.push(kind as i64);
    }
    if i == path.len() {
        return Some((main, subs));
    } else {
        return None;
    }
}

fn find_encoding(path: &Vec<Move>) -> (Vec<i64>, [Vec<Move>; 3]) {
    for s1 in (2..=6).rev() {
        for s2 in (2..=6).rev() {
            for s3 in (2..=6).rev() {
                if let Some(res) = try_size(path, [s1, s2, s3]) {
                    return res;
                }
            }
        }
    }
    return (Vec::new(), [Vec::new(), Vec::new(), Vec::new()]);
}

fn main() {
    let stdin = io::stdin();
    let mem = intcode::parse(&stdin.lock().lines().next().unwrap().unwrap());
    let map = get_map(mem.clone());
    let mut pos = (0, 0);
    let mut dir = (0, 0);
    'outer: for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] >= 2 {
                pos = (i as i64, j as i64);
                match map[i][j] {
                    2 => dir = (-1, 0),
                    3 => dir = (1, 0),
                    4 => dir = (0, 1),
                    5 => dir = (0, -1),
                    _ => {},
                }
                break 'outer;
            }
        }
    }
    let valid = |(y, x)| {
        if y >= 0 && (y as usize) < map.len() && x >= 0 && (x as usize) < map[y as usize].len() {
            return map[y as usize][x as usize] != 0;
        } else {
            return false;
        }
    };
    let mut path = Vec::new();
    loop {
        if !valid((pos.0 + dir.0, pos.1 + dir.1)) {
            let left = (-dir.1, dir.0);
            let right = (dir.1, -dir.0);
            if valid((pos.0 + left.0, pos.1 + left.1)) {
                dir = left;
                path.push(Move::Left(0));
            } else if valid((pos.0 + right.0, pos.1 + right.1)) {
                dir = right;
                path.push(Move::Right(0));
            } else {
                break;
            }
        }
        let mut i = 0;
        while valid((pos.0 + dir.0, pos.1 + dir.1)) {
            pos.0 += dir.0;
            pos.1 += dir.1;
            i += 1;
        }
        let last = path.len() - 1;
        match path[last] {
            Move::Left(x) => path[last] = Move::Left(x + i),
            Move::Right(x) => path[last] = Move::Right(x + i),
        }
    }
    let (main, subs) = find_encoding(&path);
    println!("Result: {}", notify(mem, &main, &subs));
}

