use std::fs;
use std::collections::HashSet;

pub fn run() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> String {
    let groups = make_group_set();

    let mut count = 0;
    for group in groups {
        count = count + group.len();
    }

    count.to_string()
}

fn part2() -> String {
    let groups = make_group_intersection();

    let mut count = 0;
    for group in groups {
        count = count + group.len();
    }

    count.to_string()
}

fn make_group_set() -> Vec<HashSet<char>> {
    let mut groups: Vec<HashSet<char>> = Vec::new();
    let input = get_input("./src/day6/day6.input");

    let mut group: HashSet<char> = HashSet::new();
    for s in input {
        if s.trim().len() == 0 {
            groups.push(group);
            group = HashSet::new();
            continue;
        }
        for c in s.chars() {
            group.insert(c);
        }
    }
    groups
}

fn make_group_intersection() -> Vec<HashSet<char>> {
    let mut groups: Vec<HashSet<char>> = Vec::new();
    let input = get_input("./src/day6/day6.input");

    let mut group: HashSet<char> = HashSet::new();
    let mut initialize = true;

    for s in input {
        if s.trim().len() == 0 {
            groups.push(group);
            group = HashSet::new();
            initialize = true;
            continue;
        }
        let mut person: HashSet<char> = HashSet::new();

        for c in s.chars() {
            person.insert(c);
        }
        if initialize {
            group.extend(&person);
            initialize = false;
        }
        else {
            group = group
                .intersection(&person)
                .cloned()
                .collect();
        }
    }
    groups
}

fn get_input(path: &str) -> Vec<String> {
    fs::read_to_string(path)
        .expect("No such file!")
        .lines()
        .map(|x| String::from(x))
        .collect()
}