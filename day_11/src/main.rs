use std::collections::{HashMap, HashSet};

use anyhow::{self, Context};
use pathfinding::directed::bfs;
use xmas::display_result;

fn main() -> anyhow::Result<()> {
    part_1()?;
    println!();
    part_2()?;
    Ok(())
}

fn part_1() -> anyhow::Result<()> {
    println!("Part 1:");
    let input = std::fs::read_to_string("./input.txt").context("Error reading input file.")?;

    let network = parse_network(&input);

    let mut tested_destinations = HashSet::new();
    loop {


        let last_path_node = {
            let found_path = bfs::bfs(
                &"you",
                |&n| get_successors(n, &network, &tested_destinations),
                |&n| n == "out");
            let path = match found_path {
                Some(p) => p,
                None => break,
            };

            let last_path_node = path.len() - 1;
            (path[last_path_node - 1].to_string(), path[last_path_node].to_string())
        };

        tested_destinations.insert(last_path_node);
    }

    let result = tested_destinations.len();
    display_result(&result);
    Ok(())
}

fn get_successors<'a>(from: &'a str, network: &'a Network, tested: &'a HashSet<(String, String)>) -> impl Iterator<Item = &'a str> + 'a {
    network[from]
        .iter()
        .map(|s| s.as_str())
        .filter(|&to| !tested.contains(&(from.to_string(), to.to_string())))
}

fn part_2() -> anyhow::Result<()> {
    println!("Part 2:");

    Ok(())
}

fn parse_network(input: &str) -> Network {
    let mut network = Network::new();
    for line in input.lines() {
        let (id, others) = line.split_once(": ").unwrap();

        let mut other_ids = Node::new();
        for other in others.split_whitespace() {
            other_ids.push(other.to_string());
        }

        network.insert(id.to_string(), other_ids);
    }
    network
}

type ID = String;
type Network = HashMap<ID, Node>;
type Node = Vec<ID>;
