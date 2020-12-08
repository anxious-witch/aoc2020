use std::fs;
use std::convert::TryFrom;
use std::collections::HashSet;

#[derive(Debug)]
struct State {
    acc: i64,
    ptr: i64
}

pub fn run() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> String {
    let input = get_input("./src/day8/day8.input");
    let mut visited: HashSet<i64> = HashSet::new();

    let mut state = State {
        acc: 0,
        ptr: 0
    };


    loop {
        if visited.contains(&state.ptr) {
            return state.acc.to_string();
        }
        visited.insert(state.ptr);

        let index = usize::try_from(state.ptr).unwrap();
        let line = &input[index];

        let new_state = get_instruction(line.to_string(), &state);

        state = new_state;
    }
}

fn part2() -> String {
    let input = get_input("./src/day8/day8.input");
    let mut visited: HashSet<i64> = HashSet::new();
    let mut programs: Vec<Vec<String>> = vec![];
    let program_length = input.len();


    for (i, line) in input.iter().enumerate() {
        if line.contains("nop") {
            let mut nop_to_jmp = input.clone();
            nop_to_jmp[i] = nop_to_jmp[i].replace("nop", "jmp");
            programs.push(nop_to_jmp);
        }
        if line.contains("jmp") {
            let mut jmp_to_nop = input.clone();
            jmp_to_nop[i] = jmp_to_nop[i].replace("jmp", "nop");
            programs.push(jmp_to_nop);
        }
    }

    for program in programs {
        visited.clear();

        let mut state = State {
            acc: 0,
            ptr: 0
        };

        loop {
            if visited.contains(&state.ptr) {
                // Infinite loop, invalid solution
                break;
            }
            visited.insert(state.ptr);

            let index = usize::try_from(state.ptr).unwrap();
            if index == program_length {
                // Out of index, valid solution
                return state.acc.to_string();
            }

            let line = &program[index];

            let new_state = get_instruction(line.to_string(), &state);

            state = new_state;
        }
    }

    return "No result".to_string();
}

fn get_instruction(line: String, state: &State) -> State {
    let split = &line.split_whitespace()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    let instruction = &split[0];
    let operand = split[1].parse::<i64>().unwrap();

    let new_state = match &instruction[..] {
        "nop"   => State {
            acc: state.acc,
            ptr: state.ptr + 1
        },
        "acc"   => State {
            acc: state.acc + operand,
            ptr: state.ptr + 1
        },
        "jmp"   => State {
            acc: state.acc,
            ptr: state.ptr + operand
        },
        _       => State {
            acc: state.acc,
            ptr: state.ptr
        }
    };

    new_state
}

fn get_input(path: &str) -> Vec<String> {
    fs::read_to_string(path)
        .expect("No such file!")
        .lines()
        .map(|x| String::from(x))
        .collect()
}