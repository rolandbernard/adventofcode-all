
use std::io;
use std::io::prelude::*;
use std::collections::*;

#[path ="../intcode/intcode.rs"]
mod intcode;

#[derive(Debug)]
struct Room {
    conn: [Option<usize>; 4],
    items: Vec<usize>,
}

type Station = HashMap<usize, Room>;

fn give_command(mem: &mut intcode::IntMem, cpu: &mut intcode::IntState, cmd: &str) {
    let mut nums = cmd.chars().map(|c| c as u8 as i64).collect::<VecDeque<_>>();
    while !cpu.halt && !nums.is_empty() {
        intcode::run_instr(mem, cpu, || nums.pop_front().unwrap(), |_| {});
    }
}

fn get_room_data(mem: &mut intcode::IntMem, cpu: &mut intcode::IntState) -> Option<String> {
    let mut out = String::new();
    let mut i = 0;
    while !cpu.halt && i < 100000 && !out.ends_with("Command?\n") {
        intcode::run_instr(mem, cpu, || panic!(), |c| out.push(c as u8 as char));
        i += 1;
    }
    if out.ends_with("Command?\n") {
        return Some(out);
    } else {
        return None;
    }
}

fn get_password(mem: &mut intcode::IntMem, cpu: &mut intcode::IntState) -> Option<i64> {
    let mut res = None;
    let mut out = String::new();
    let mut i = 0;
    while !cpu.halt && i < 100000 && res == None && out != "Command?" {
        intcode::run_instr(mem, cpu, || panic!(), |x| {
            let c = x as u8 as char;
            if c.is_whitespace() {
                res = out.parse::<i64>().ok();
                out.clear();
            } else {
                out.push(c);
            }
        });
        i += 1;
    }
    return res;
}

fn can_take(mut mem: intcode::IntMem, mut cpu: intcode::IntState, itm: &str) -> bool {
    give_command(&mut mem, &mut cpu, "take ");
    give_command(&mut mem, &mut cpu, itm);
    give_command(&mut mem, &mut cpu, "\n");
    if get_room_data(&mut mem, &mut cpu) == None {
        return false;
    } else {
        let mut valid = false;
        for i in 0..4 {
            let mut nm = mem.clone();
            let mut nc = cpu.clone();
            give_command(&mut nm, &mut nc, to_dir(i));
            give_command(&mut nm, &mut nc, "\n");
            if let Some(desc) = get_room_data(&mut nm, &mut nc) {
                let lines = desc.trim().split("\n").collect::<Vec<_>>();
                if lines[0].chars().next().unwrap_or('?') == '=' {
                    valid = true;
                }
            }
        }
        return valid;
    }
}

fn to_dir(num: usize) -> &'static str {
    return match num {
        0 => "north",
        1 => "south",
        2 => "east",
        3 => "west",
        _ => panic!(),
    }
}

fn explore(
    mut mem: intcode::IntMem, mut cpu: intcode::IntState, station: &mut Station,
    rooms: &mut HashMap<String, usize>, items: &mut HashMap<String, usize>, item_list: &mut Vec<String>
) -> Option<usize> {
    if let Some(desc) = get_room_data(&mut mem, &mut cpu) {
        let lines = desc.trim().split("\n").collect::<Vec<_>>();
        if lines[0].chars().next().unwrap_or('?') == '=' {
            let name = lines[0].trim_matches(|c| c == '=' || c == ' ');
            if !rooms.contains_key(name) {
                let id = rooms.len();
                rooms.insert(name.to_owned(), id);
                let mut itms = Vec::new();
                if let Some(mut idx) = lines.iter().position(|&r| r == "Items here:") {
                    idx += 1;
                    while lines[idx] != "" {
                        if can_take(mem.clone(), cpu.clone(), &lines[idx][2..]) {
                            let id;
                            if items.contains_key(&lines[idx][2..]) {
                                id = items[&lines[idx][2..]];
                            } else {
                                id = items.len();
                                items.insert(lines[idx][2..].to_owned(), id);
                                item_list.push(lines[idx][2..].to_owned());
                            }
                            itms.push(id);
                        }
                        idx += 1;
                    }
                }
                station.insert(id, Room { conn: [None, None, None, None], items: itms });
                for i in 0..4 {
                    let mut nm = mem.clone();
                    let mut nc = cpu.clone();
                    give_command(&mut nm, &mut nc, to_dir(i));
                    give_command(&mut nm, &mut nc, "\n");
                    station.get_mut(&id).unwrap().conn[i] = explore(nm, nc, station, rooms, items, item_list);
                }
            }
            return Some(rooms[name]);
        }
    }
    return None;
}

fn collect_all<'a>(
    pos: usize, station: &Station, mut seen: usize, count: usize, target: usize,
    items: &'a Vec<String>, cache: &mut HashSet<(usize, usize)>
) -> Option<(usize, Vec<&'a str>)> {
    if !cache.contains(&(pos, seen)) {
        cache.insert((pos, seen));
        let mut itms = Vec::new();
        for &i in &station[&pos].items {
            if (seen & (1 << i)) == 0 {
                seen |= 1 << i;
                itms.push("take ");
                itms.push(&items[i]);
                itms.push("\n");
            }
        }
        if seen == (1 << count) - 1 && station[&pos].conn.iter().any(|x| x == &Some(target)) {
            return Some((pos, itms));
        } else {
            for (i, &d) in station[&pos].conn.iter().enumerate() {
                if let Some(next) = d {
                    if let Some((p, mut res)) = collect_all(next, station, seen, count, target, items, cache) {
                        itms.push(to_dir(i));
                        itms.push("\n");
                        itms.append(&mut res);
                        return Some((p, itms));
                    }
                }
            }
            return None;
        }
    } else {
        return None;
    }
}

fn main() {
    let stdin = io::stdin();
    let mut mem = intcode::parse(&stdin.lock().lines().next().unwrap().unwrap());
    let mut cpu = intcode::IntState::new();
    let mut station = HashMap::new();
    let mut rooms = HashMap::new();
    let mut items = HashMap::new();
    let mut item_list = Vec::new();
    let entry = explore(mem.clone(), cpu.clone(), &mut station, &mut rooms, &mut items, &mut item_list).unwrap();
    let target = rooms["Pressure-Sensitive Floor"];
    let mut cache = HashSet::new();
    let (pos, todo) = collect_all(entry, &station, 0, items.len(), target, &item_list, &mut cache).unwrap();
    for s in todo {
        give_command(&mut mem, &mut cpu, &s);
    }
    let dir = to_dir(station[&pos].conn.iter().position(|x| x == &Some(target)).unwrap());
    for i in 0..1 << items.len() {
        let mut nm = mem.clone();
        let mut nc = cpu.clone();
        for j in 0..items.len() {
            if (i & (1 << j)) != 0 {
                give_command(&mut nm, &mut nc, "drop ");
                give_command(&mut nm, &mut nc, &item_list[j]);
                give_command(&mut nm, &mut nc, "\n");
            }
        }
        give_command(&mut nm, &mut nc, dir);
        give_command(&mut nm, &mut nc, "\n");
        if let Some(passwd) = get_password(&mut nm, &mut nc) {
            println!("Result: {}", passwd);
            break;
        }
    }
}

