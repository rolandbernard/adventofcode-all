
use std::io;
use std::io::prelude::*;
use std::collections::*;

#[path ="../intcode/intcode.rs"]
mod intcode;

fn movement(mem: &mut intcode::IntMem, cpu: &mut intcode::IntState, inp: i64) -> i64 {
    let mut out = None;
    while !cpu.halt && out == None {
        intcode::run_instr(mem, cpu, || inp, |x| { out = Some(x); });
    }
    return out.unwrap_or(0);
}

fn explore_map(
    pos: (i64, i64), seen: &mut HashSet<(i64, i64)>, target: &mut (i64, i64), mem: &intcode::IntMem, cpu: &intcode::IntState
) {
    if !seen.contains(&pos) {
        seen.insert(pos);
        for i in 1..=4 {
            let mut tmp_mem = mem.clone();
            let mut tmp_cpu = cpu.clone();
            let res = movement(&mut tmp_mem, &mut tmp_cpu, i);
            let new_pos = match i {
                1 => (pos.0 + 1, pos.1),
                2 => (pos.0 - 1, pos.1),
                3 => (pos.0, pos.1 + 1),
                4 => (pos.0, pos.1 - 1),
                _ => pos,
            };
            if res != 0 {
                if res == 2 {
                    *target = new_pos;
                }
                explore_map(new_pos, seen, target, &tmp_mem, &tmp_cpu);
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mem = intcode::parse(&stdin.lock().lines().next().unwrap().unwrap());
    let cpu = intcode::IntState::new();
    let mut seen = HashSet::new();
    let mut target = (0, 0);
    explore_map((0, 0), &mut seen, &mut target, &mem, &cpu);
    let mut queue = VecDeque::new();
    let mut dist = HashMap::new();
    queue.push_back((0, (0, 0)));
    while let Some((d, (x, y))) = queue.pop_front() {
        if !dist.contains_key(&(x, y)) {
            dist.insert((x, y), d);
            if target == (x, y) {
                break;
            } else if seen.contains(&(x, y)) {
                queue.push_back((d + 1, (x + 1, y)));
                queue.push_back((d + 1, (x - 1, y)));
                queue.push_back((d + 1, (x, y + 1)));
                queue.push_back((d + 1, (x, y - 1)));
            }
        }
    }
    println!("Result: {}", dist[&target]);
}

