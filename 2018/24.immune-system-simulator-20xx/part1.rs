
use std::io;
use std::io::prelude::*;
use std::collections::*;

struct Group {
    army: i64,
    units: i64,
    points: i64,
    attack: i64,
    initia: i64,
    kind: String,
    weak: HashSet<String>,
    immune: HashSet<String>,
    attacker: bool,
    target: Option<usize>,
}

impl Group {
    fn parse(army: i64, s: &str) -> Group {
        let splt = s.split(|c| c == '(' || c == ')').collect::<Vec<_>>();
        let fst = splt[0].split(" ").collect::<Vec<_>>();
        let lst;
        if splt.len() == 3 {
            lst = splt[2].split(" ").collect::<Vec<_>>();
        } else {
            lst = splt[0].split(" ").skip(6).collect::<Vec<_>>();
        }
        let mut weak = HashSet::new();
        let mut immune = HashSet::new();
        if splt.len() == 3 {
            for t in splt[1].split("; ") {
                let splt = t.split(" to ").collect::<Vec<_>>();
                let types = splt[1].split(", ").map(|x| x.to_owned()).collect::<HashSet<_>>();
                if splt[0] == "weak" {
                    weak = types;
                } else {
                    immune = types;
                }
            }
        }
        return Group {
            army,
            units: fst[0].parse::<i64>().unwrap(),
            points: fst[4].parse::<i64>().unwrap(),
            attack: lst[6].parse::<i64>().unwrap(),
            initia: lst[11].parse::<i64>().unwrap(),
            kind: lst[7].to_owned(),
            weak, immune, attacker: false, target: None,
        };
    }

    fn power(&self) -> i64 {
        return self.units * self.attack;
    }

    fn damage_to(&self, other: &Group) -> i64 {
        if other.immune.contains(&self.kind) {
            return 0;
        } else if other.weak.contains(&self.kind) {
            return 2 * self.power();
        } else {
            return self.power();
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut groups = Vec::new();
    let mut army = 0;
    for l in stdin.lock().lines() {
        let line = l.unwrap();
        if line.ends_with(":") {
            army += 1;
        } else if !line.is_empty() {
            groups.push(Group::parse(army, &line));
        }
    }
    let mut sel_ord = (0..groups.len()).collect::<Vec<_>>();
    let mut att_ord = (0..groups.len()).collect::<Vec<_>>();
    while sel_ord.iter().any(|&x| groups[x].army == 1) && sel_ord.iter().any(|&x| groups[x].army == 2) {
        sel_ord.sort_by_key(|&x| (-groups[x].power(), -groups[x].initia));
        att_ord.sort_by_key(|&x| -groups[x].initia);
        for &i in &sel_ord {
            groups[i].target = (0..groups.len())
                .filter(|&x| !groups[x].attacker && groups[x].army != groups[i].army && groups[i].damage_to(&groups[x]) > 0)
                .max_by_key(|&x| (groups[i].damage_to(&groups[x]), groups[x].power(), groups[x].initia));
            if let Some(j) = groups[i].target {
                groups[j].attacker = true;
            }
        }
        for &i in &att_ord {
            if let Some(j) = groups[i].target {
                groups[j].units = (groups[j].units - groups[i].damage_to(&groups[j]) / groups[j].points).max(0);
                groups[i].target = None;
            }
            groups[i].attacker = false;
        }
        sel_ord.retain(|&x| groups[x].units > 0);
        att_ord.retain(|&x| groups[x].units > 0);
    }
    println!("Result: {}", sel_ord.iter().map(|&x| groups[x].units).sum::<i64>());
}

