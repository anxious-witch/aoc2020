use std::fs;
use std::collections::HashSet;
use std::iter::FromIterator;

pub fn run() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> String {
    let input = get_input("./src/day1/day1.input");
    let set = HashSet::<&i32>::from_iter(&input);

    for num in &input {
        let x = 2020 - num;
        if set.contains(&x) {
            return (num * x).to_string();
        }
    }

    return String::from("No result");
}

fn part2() -> String {
    let input = get_input("./src/day1/day1.input");

    for a in &input {
        for b in &input {
            for c in &input {
                if a + b + c == 2020 {
                    return (a * b * c).to_string();
                }
            }
        }
    }

    return String::from("No result");
}

fn get_input(path: &str) -> Vec<i32> {
    fs::read_to_string(path)
        .expect("No such file!")
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}