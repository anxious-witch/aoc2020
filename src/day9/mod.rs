use std::fs;
use std::collections::HashSet;

pub fn run() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> String {
    let input = get_input("./src/day9/day9.input");

    let len = 25;

    for i in len..input.len() {
        let preamble_sums = get_preamble(&input, i - len, len);

        if !preamble_sums.contains(&input[i]) {
            return input[i].to_string();
        }
    }
    return "No result".to_string();
}

fn part2() -> String {
    let input = get_input("./src/day9/day9.input");

    let len = 25;
    let mut invalid = 0;
    let mut index = 0;

    for i in len..input.len() {
        let preamble_sums = get_preamble(&input, i - len, len);

        if !preamble_sums.contains(&input[i]) {
            invalid = input[i];
            index = i;
            break;
        }
    }

    for i in 0..index {
        let mut range: Vec<u128> = vec![];

        for j in i..index {
            let sum: u128 = range.iter().sum();
            if sum == invalid {
                return (
                    range.iter().min().unwrap_or(&0) + 
                    range.iter().max().unwrap_or(&0)
                ).to_string();
            }
            if sum > invalid {
                break;
            }
            range.push(input[j]);
        }
    }

    return "No result".to_string();
}

fn get_preamble(list: &Vec<u128>, index: usize, len: usize) -> HashSet<u128> {
    let mut preamble_sums: HashSet<u128> = HashSet::new();

    for i in (0 + index)..(len + index) {
        for j in i..(len + index) {
            if i == j {
                continue;
            }

            let x = list[i] + list[j];
            preamble_sums.insert(x);
        }
    }

    preamble_sums
}

fn get_input(path: &str) -> Vec<u128> {
    fs::read_to_string(path)
        .expect("No such file!")
        .lines()
        .map(|x| x.parse::<u128>().unwrap())
        .collect()
}