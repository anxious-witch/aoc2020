use std::fs;
use std::collections::HashMap;
use petgraph::{Graph};
use petgraph::prelude::NodeIndex;
use petgraph::visit::Dfs;
use petgraph::Direction;

pub fn run() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> String {
    let (graph, index_map) = make_bag_graph();
    let destination = index_map["shiny gold"];
    let mut can_contain = 0;

    for source in index_map.values() {
        if *source == destination {
            continue;
        }
        let mut dfs = Dfs::new(&graph, *source);

        while let Some(x) = dfs.next(&graph) {
            if x == destination {
                can_contain = can_contain + 1;
            }
        }
    }

    can_contain.to_string()
}

fn part2() -> String {
    let (graph, index_map) = make_bag_graph();
    let start = index_map["shiny gold"];

    get_bags(&graph,  start).to_string()
}

fn get_bags(graph: &Graph::<String, i32>, node: NodeIndex) -> i32 {
    let mut weight = 0;

    for neighbor in graph.neighbors_directed(node, Direction::Outgoing) {
        for edge in graph.edges_connecting(node, neighbor) {
            let w = edge.weight();
            weight += w + w * get_bags(&graph, neighbor);
        }
    }

    weight
}

fn make_bag_graph() -> (Graph::<String, i32>, HashMap::<String, NodeIndex>) {
    let mut graph = Graph::<String, i32>::new();

    let mut index_map = HashMap::<String, NodeIndex>::new();
    let rules = get_input("./src/day7/day7.input");

    for rule in &rules {
        let bag: String = rule[0].split_whitespace()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()[0..2]
            .join(" ")
            .to_string();

        let index = graph.add_node(bag.clone());
        index_map.insert(bag, index);
    }

    for rule in &rules {
        if rule[1] == "no other bags." {
            continue;
        }
        let bag: String = rule[0].split_whitespace()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()[0..2]
            .join(" ")
            .to_string();

        let contains = rule[1].split(',')
            .map(|x| x.trim().to_string())
            .collect::<Vec<String>>();

        for c in contains {
            let edge = c.split_whitespace()
                .map(|x| x.to_string())
                .collect::<Vec<String>>();

            let amount = edge[0].parse::<i32>().unwrap();
            let child_bag = edge[1..3].join(" ").to_string();

            graph.add_edge(
                index_map[&bag],
                index_map[&child_bag],
                amount
            );
        }
    }

    (graph, index_map)
}

fn get_input(path: &str) -> Vec<Vec<String>> {
    let input: Vec<String> = fs::read_to_string(path)
        .expect("No such file!")
        .lines()
        .map(|x| String::from(x))
        .collect();

    let mut vec: Vec<Vec<String>> = vec![];

    for s in input {
        vec.push(s.split("contain")
            .map(|x| x.trim().to_string())
            .collect()
        )
    }

    vec
}