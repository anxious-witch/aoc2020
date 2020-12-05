use std::fs;

pub fn run() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> String {
    return find_trees(1, 3).to_string();
}

fn part2() -> String {
    return (
        find_trees(1, 1)
        * find_trees(1, 3)
        * find_trees(1, 5)
        * find_trees(1, 7)
        * find_trees(2, 1)
    ).to_string();
}

fn find_trees(down: usize, right: usize) -> u64 {
    let input = get_input("./src/day3/day3.input");
    let mut pos = (0, 0);
    let mut trees = 0;
    let tree = '#';

    let height = input.len();
    let width = input[0].len();

    loop {
        if pos.0 >= height {
            break;
        }
        if pos.1 >= width {
            pos = (pos.0, pos.1 % width)
        }

        if input[pos.0][pos.1] == tree {
            trees = trees + 1;
        }

        pos = (pos.0 + down, pos.1 + right);
    }

    return trees;
}

fn get_input(path: &str) -> Vec<Vec<char>> {
    fs::read_to_string(path)
        .expect("No such file!")
        .lines()
        .map(|x| x.chars().collect())
        .collect()
}