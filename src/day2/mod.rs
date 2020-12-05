use std::fs;
use regex;

pub fn run() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> String {
    let mut correct = 0;

    let input = get_input("./src/day2/day2.input");
    let re = regex::Regex::new(r"(?P<range>\d+-\d+) (?P<char>[A-z]): (?P<password>[A-z]+)").unwrap();

    for password in input {
        let capture = re.captures(&password).unwrap();

        if let [range_start, range_end] = String::from(&capture["range"])
            .split('-')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>()
            [..] {

                let rule = String::from(&capture["char"]);
                let pass = String::from(&capture["password"]);

                let char_count = pass.matches(&rule).count();

                if char_count >= range_start && char_count <= range_end {
                    correct = correct + 1;
                }
            }

    }

    return correct.to_string();
}

fn part2() -> String {
    let mut correct = 0;

    let input = get_input("./src/day2/day2.input");
    let re = regex::Regex::new(r"(?P<range>\d+-\d+) (?P<char>[A-z]): (?P<password>[A-z]+)").unwrap();

    for password in input {
        let capture = re.captures(&password).unwrap();

        if let [index_start, index_end] = String::from(&capture["range"])
            .split('-')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>()
            [..] {

                let rule = capture["char"].chars().next().unwrap();
                let chars: Vec<char> = capture["password"].chars().collect();

                let mut index_matches = 0;

                if chars[index_start - 1] == rule {
                    index_matches = index_matches + 1;
                }

                if chars[index_end - 1] == rule {
                    index_matches = index_matches + 1;
                }

                if index_matches == 1 {
                    correct = correct + 1;
                }
            }

    }

    return correct.to_string();
}

fn get_input(path: &str) -> Vec<String> {
    fs::read_to_string(path)
        .expect("No such file!")
        .lines()
        .map(|x| String::from(x))
        .collect()
}