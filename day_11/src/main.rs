use std::collections::HashMap;
use anyhow::{self, Context};
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

    let result = get_combination_count("you", &network);
    display_result(&result);
    Ok(())
}

fn part_2() -> anyhow::Result<()> {
    println!("Part 2:");
    let input = std::fs::read_to_string("./input.txt").context("Error reading input file.")?;

    let network = parse_network(&input);

    let result = get_combination_count_complete(&network);
    display_result(&result);

    Ok(())
}

fn get_combination_count(from: &str, network: &Network) -> usize {
    if from == "out" {
        return 1;
    }

    network[from]
        .iter()
        .map(|n| get_combination_count(n, network))
        .sum()
}

fn get_combination_count_complete(network: &Network) -> usize {
    get_combination_count_cached(network, CacheKey::new("svr".to_string()), &mut HashMap::new())
}

fn get_combination_count_cached(network: &Network, next_key: CacheKey, cache: &mut HashMap<CacheKey, usize>) -> usize {
    if let Some(&cached_val) = cache.get(&next_key) {
        return cached_val;
    }

    let node = next_key.node.as_str();
    if node == "out" {
        return if next_key.has_dac && next_key.has_fft { 1 } else { 0 };
    }

    let found_node = network.get(node);
    if found_node.is_none() {
        panic!("Node \"{node}\" not found");
    }

    let sum = found_node
        .unwrap()
        .iter()
        .map(|n| get_combination_count_cached(
            network,
            CacheKey {
                node: n.clone(),
                has_dac: next_key.has_dac || node == "dac",
                has_fft: next_key.has_fft || node == "fft",
            },
            cache))
        .sum();

    cache.insert(next_key.clone(), sum);
    sum
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

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct CacheKey {
    pub node: String,
    pub has_dac: bool,
    pub has_fft: bool,
}

impl CacheKey {
    pub fn new(node: String) -> Self {
        Self { node, has_dac: false, has_fft: false }
    }
}
