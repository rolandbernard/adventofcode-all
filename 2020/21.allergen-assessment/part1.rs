
use std::io;
use std::io::prelude::*;
use std::collections::*;

// mxmxvkd kfcds sqjhc nhms (contains dairy, fish)

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
    let mut with_allergies = HashSet::new();
    for p in pos.values() {
        for a in p {
            with_allergies.insert(a);
        }
    }
    let mut count = 0;
    for food in &foods {
        for f in &food.0 {
            if !with_allergies.contains(f) {
                count += 1;
            }
        }
    }
    println!("Result: {}", count);
}

