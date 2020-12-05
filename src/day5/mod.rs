use std::fs;
use std::cmp;
use std::collections::HashSet;

pub fn run() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> String {
    let input = get_input("./src/day5/day5.input");
    let mut max = 0;

    for pass in input {
        let [row_instructions, column_instructions] = [
            pass[..7].to_string(),
            pass[7..].to_string()
        ];

        let x = get_row(row_instructions) * 8 + get_column(column_instructions);
        max = cmp::max(max, x);
    }
    max.to_string()
}

fn part2() -> String {
    let input = get_input("./src/day5/day5.input");
    let mut passes = HashSet::<i32>::new();
    let mut max = 0;
    let mut min = 2 << 32 - 1;

    for pass in input {
        let [row_instructions, column_instructions] = [
            pass[..7].to_string(),
            pass[7..].to_string()
        ];

        let x = get_row(row_instructions) * 8 + get_column(column_instructions);
        max = cmp::max(max, x);
        min = cmp::min(min, x);
        passes.insert(x);
    }

    let sentinel: HashSet<i32> = (min..max).collect();
    let diff = sentinel.difference(&passes);

    for d in diff {
        let x = d - 1;
        let y = d + 1;

        if passes.contains(&x) && passes.contains(&y) {
            return d.to_string();
        }
    }

    String::from("No results")
}

fn get_row(instructions: String) -> i32 {
    let mut start = 0;
    let mut end = 127;

    for chr in instructions.chars() {
        let middle = (start + end) / 2 + 1;
        match chr {
            'B' => start = middle,
            'F' => end = middle - 1,
            _   => ()
        }
    }
    start
}

fn get_column(instructions: String) -> i32 {
    let mut start = 0;
    let mut end = 7;

    for chr in instructions.chars() {
        let middle = (start + end) / 2 + 1;
        match chr {
            'R' => start = middle,
            'L' => end = middle - 1,
            _   => ()
        }
    }
    start
}

fn get_input(path: &str) -> Vec<String> {
    fs::read_to_string(path)
        .expect("No such file!")
        .lines()
        .map(|x| String::from(x))
        .collect()
}