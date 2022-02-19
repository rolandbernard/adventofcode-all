
use std::io;
use std::io::prelude::*;
use std::collections::*;

fn is_valid(pass: &HashSet<String>) -> bool {
    return pass.contains("byr")
        && pass.contains("iyr")
        && pass.contains("eyr")
        && pass.contains("hgt")
        && pass.contains("hcl")
        && pass.contains("ecl")
        && pass.contains("pid");
}

fn main() {
    let stdin = io::stdin();
    let mut passport = HashSet::new();
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
                passport.insert(splt[0].to_owned());
            }
        }
    }
    if is_valid(&passport) {
        count += 1;
    }
    println!("Result: {}", count);
}

