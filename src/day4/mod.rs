use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use regex::Regex;

pub fn run() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> String {
    let passports = make_passport_map();
    let mut valid_passports = 0;

    let required_fields: HashSet<String> = [
        "byr",
        "iyr",
        "eyr",
        "hgt",
        "hcl",
        "ecl",
        "pid",
    ].iter().map(|x| x.to_string()).collect();

    for passport in passports {
        let keys: HashSet<String> = passport
            .keys()
            .map(|x| x.to_string())
            .collect();

        if keys.is_superset(&required_fields) {
            valid_passports = valid_passports + 1;
        }
    }

    return valid_passports.to_string();
}

fn part2() -> String {
    let passports = make_passport_map();
    let mut valid_passports = 0;

    let required_fields: HashSet<String> = [
        "byr",
        "iyr",
        "eyr",
        "hgt",
        "hcl",
        "ecl",
        "pid",
    ].iter().map(|x| x.to_string()).collect();

    let re_hcl = Regex::new(r"#[0-9a-f]{6}").unwrap();
    let re_pid = Regex::new(r"^\d{9}$").unwrap();

    for passport in passports {
        let keys: HashSet<String> = passport
            .keys()
            .map(|x| x.to_string())
            .collect();

        if keys.is_superset(&required_fields) {
            match passport["byr"].parse::<i32>() {
                Ok(n) => match n {
                    1920..=2002 => (),
                    _ => continue
                }
                _ => continue
            }
            match passport["iyr"].parse::<i32>() {
                Ok(n) => match n {
                    2010..=2020 => (),
                    _ => continue
                }
                _ => continue
            }
            match passport["eyr"].parse::<i32>() {
                Ok(n) => match n {
                    2020..=2030 => (),
                    _ => continue
                }
                _ => continue
            }
            match &passport["ecl"][..] {
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => (),
                _ => continue
            }
            if !re_hcl.is_match(&passport["hcl"]) {
                continue;
            }
            if !re_pid.is_match(&passport["pid"]) {
                continue;
            }

            if passport["hgt"].contains("in") {
                let height_in = passport["hgt"][0..2].parse::<i32>();
                match height_in {
                    Ok(n) => match n {
                        59..=76 => (),
                        _ => continue
                    }
                    _ => continue
                }
            }
            else if passport["hgt"].contains("cm") {
                let height_cm = passport["hgt"][0..3].parse::<i32>();
                match height_cm {
                    Ok(n) => match n {
                        150..=193 => (),
                        _ => {
                            continue
                        }
                    }
                    _ => continue
                }
            }
            else {
                continue;
            }

            valid_passports = valid_passports + 1;
        }
    }

    return valid_passports.to_string();
}

fn make_passport_map() -> Vec<HashMap<String, String>> {
    let mut passports: Vec<HashMap<String, String>> = Vec::new();
    let input = get_input("./src/day4/day4.input");

    let mut passport: HashMap<String, String> = HashMap::new();
    for s in input {
        if s.trim().len() == 0 {
            passports.push(passport);
            passport = HashMap::new();
            continue;
        }
        for parts in s.split(' ') {
            let split = parts
                .split(':')
                .collect::<Vec<&str>>();
            passport.insert(
                split[0].to_string(),
                split[1].to_string()
            );
        }
    }

    passports
}

fn get_input(path: &str) -> Vec<String> {
    fs::read_to_string(path)
        .expect("No such file!")
        .lines()
        .map(|x| String::from(x))
        .collect()
}