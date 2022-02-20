
use std::io;
use std::io::prelude::*;
use std::collections::*;

fn main() {
    let stdin = io::stdin();
    let mut foods = Vec::new();
    let mut ingred = HashSet::new();
    let mut allerg = HashSet::new();
    for line in stdin.lock().lines().map(|x| x.unwrap()) {
        let len = line.len();
        let ingr_end = line.find('(').unwrap();
        let mut ingreds = HashSet::new();
        let mut allergs = HashSet::new();
        for ingr in line[0..ingr_end - 1].split(" ") {
            ingred.insert(ingr.to_owned());
            ingreds.insert(ingr.to_owned());
        }
        for aller in line[ingr_end + 10..len - 1].split(", ") {
            allerg.insert(aller.to_owned());
            allergs.insert(aller.to_owned());
        }
        foods.push((ingreds, allergs));
    }
    let mut pos = HashMap::new();
    for alle in &allerg {
        pos.insert(alle.to_owned(), ingred.clone());
    }
    for food in &foods {
        for alle in &food.1 {
            pos.get_mut(alle).unwrap().retain(|x| food.0.contains(x));
        }
    }
    let mut unknown = pos.into_iter().collect::<Vec<_>>();
    let mut known = Vec::new();
    loop {
        let mut change = false;
        for i in 0..unknown.len() {
            if unknown[i].1.len() == 1 {
                let v = unknown[i].1.iter().next().unwrap().to_owned();
                for j in 0..unknown.len() {
                    unknown[j].1.remove(&v);
                }
                known.push((unknown[i].0.to_owned(), v));
                change = true;
            }
        }
        if !change {
            break;
        }
    }
    known.sort();
    println!("Result: {}", known.into_iter().map(|(_, i)| i).collect::<Vec<_>>().join(","));
}

