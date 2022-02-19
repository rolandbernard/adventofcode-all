
use std::io;
use std::io::prelude::*;
use std::collections::*;

fn is_valid(pass: &HashMap<String, String>) -> bool {
    if !(
        pass.contains_key("byr")
        && pass.contains_key("iyr")
        && pass.contains_key("eyr")
        && pass.contains_key("hgt")
        && pass.contains_key("hcl")
        && pass.contains_key("ecl")
        && pass.contains_key("pid")
    ) {
        return false;
    }
    let byr = pass["byr"].parse::<usize>().unwrap_or(0);
    if byr < 1920 || byr > 2002 {
        return false;
    }
    let iyr = pass["iyr"].parse::<usize>().unwrap_or(0);
    if iyr < 2010 || iyr > 2020 {
        return false;
    }
    let eyr = pass["eyr"].parse::<usize>().unwrap_or(0);
    if eyr < 2020 || eyr > 2030 {
        return false;
    }
    let hgt = &pass["hgt"];
    let hgtn = hgt[0..hgt.len() - 2].parse::<usize>().unwrap_or(0);
    if &hgt[hgt.len() - 2..] == "cm" {
        if hgtn < 150 || hgtn > 193 {
            return false;
        }
    } else if &hgt[hgt.len() - 2..] == "in" {
        if hgtn < 59 || hgtn > 76 {
            return false;
        }
    } else {
        return false;
    }
    let hcl = &pass["hcl"];
    if hcl.len() != 7 || hcl.chars().next().unwrap() != '#' || hcl.chars().skip(1).any(|c| !c.is_digit(16)) {
        return false;
    }
    let ecl = &pass["ecl"];
    if ecl != "amb" && ecl != "blu" && ecl != "brn" && ecl != "gry" && ecl != "grn" && ecl != "hzl" && ecl != "oth" {
        return false;
    }
    let pid = &pass["pid"];
    if pid.len() != 9 || pid.chars().any(|c| !c.is_digit(10)) {
        return false;
    }
    return true;
}

fn main() {
    let stdin = io::stdin();
    let mut passport = HashMap::new();
    let mut count = 0;
    for l in stdin.lock().lines() {
        let line = l.unwrap();
        if line == "" {
            if is_valid(&passport) {
                count += 1;
            }
            passport.clear();
        } else {
            for field in line.split(" ") {
                let splt = field.split(":").collect::<Vec<_>>();
                passport.insert(splt[0].to_owned(), splt[1].to_owned());
            }
        }
    }
    if is_valid(&passport) {
        count += 1;
    }
    println!("Result: {}", count);
}

