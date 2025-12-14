use std::str::FromStr;

use anyhow::{self, Context};
use pathfinding::directed::dijkstra;
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

    let machines = input
        .lines()
        .map(SimpleMachine::from_str)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    // for machine in machines {
    //     println!("{:?}", machine.find_shortest_configuration());
    // }

    let result: u64 = machines
        .iter()
        .map(|m| m.find_shortest_configuration().unwrap())
        .sum();
    display_result(&result);
    Ok(())
}

fn part_2() -> anyhow::Result<()> {
    println!("Part 2:");
    let input = std::fs::read_to_string("./input.txt").context("Error reading input file.")?;

    let machines = input
        .lines()
        .map(JoltageMachine::from_str)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();


    let mut result = 0;
    for machine in machines {
        let partial_result = machine.find_shortest_configuration().unwrap();
        println!("{}", partial_result);
        result += partial_result;
    }

    // let result: u64 = machines
    //     .iter()
    //     .map(|m| m.find_shortest_configuration().unwrap())
    //     .sum();
    display_result(&result);
    Ok(())
}

type SimplePathNode = (u64, Option<usize>);

#[derive(Debug)]
struct SimpleMachine {
    target: u64,
    buttons: Vec<u64>,
}

impl SimpleMachine {
    fn find_shortest_configuration(&self) -> Option<u64> {
        let path = dijkstra::dijkstra(
            &(0, None),
            |p: &(u64, Option<usize>)| self.get_successors(p.0, p.1),
            |p| p.0 == self.target);
        path.map(|p| p.0.len() as u64 - 1)
    }
    
    fn get_successors(&self, from: u64, previous: Option<usize>) -> impl Iterator<Item = (SimplePathNode, u64)> + '_ {
        (0..self.buttons.len())
            .filter(move |&i| previous.is_none_or(|p| p != i))
            .map(move |i| ((from ^ self.buttons[i], Some(i)), 1))
    }
}

impl FromStr for SimpleMachine {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();

        let target = split
            .next()
            .unwrap()
            .trim_start_matches('[')
            .trim_end_matches(']')
            .char_indices()
            .fold(0, |value, (i, ch)| {
                if ch != '.' {
                    value | (1 << i)
                } else {
                    value
                }
            });

        let mut buttons = Vec::new();
        loop {
            let section = split.next();
            if section.is_none() {
                panic!("no joltages");
            }

            let btns = section.unwrap();
            if btns.starts_with('{') {
                break;
            }

            let mut button_group = 0;
            for btn in btns.trim_start_matches('(').trim_end_matches(')').split(',') {
                // println!("{}", btn);
                button_group |= 1 << btn.parse::<u64>().unwrap();
            }

            buttons.push(button_group);
        };
        
        Ok(Self {
            target,
            buttons,
        })
    }
}

#[derive(Debug)]
struct JoltageMachine {
    target: Vec<u32>,
    buttons: Vec<Vec<usize>>,
}

impl JoltageMachine {
    fn find_shortest_configuration(&self) -> Option<u64> {
        // println!("{:?}", self);
        let start = vec![0u32; self.target.len()];
        let path = dijkstra::dijkstra(
            &start,
            |p: &Vec<u32>| self.get_successors(p),
            |p| p == &self.target);
        path.map(|p| p.0.len() as u64 - 1)
    }
    
    fn get_successors(&self, from: &[u32]) -> Vec<(Vec<u32>, u32)> {
        let buttons = &self.buttons;
        let target = &self.target;

        (0..buttons.len())
            .filter_map(move |i| {
                let mut new_state = from.to_vec();
                let button = &buttons[i];
                for &j in button {
                    let new_value = from[j] + 1;
                    if new_value > target[j] {
                        return None;
                    }

                    new_state[j] = new_value;
                }

                assert_eq!(new_state.len(), from.len());
                Some((new_state, 1))
            })
            .collect()
    }
}

impl FromStr for JoltageMachine {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();

        split.next();

        let mut buttons = Vec::new();
        let joltages_s = loop {
            let section = split.next();
            if section.is_none() {
                panic!("no joltages");
            }

            let btns = section.unwrap();
            if btns.starts_with('{') {
                break btns;
            }

            let mut button_group = Vec::new();
            for btn in btns.trim_start_matches('(').trim_end_matches(')').split(',') {
                button_group.push(btn.parse::<usize>().unwrap());
            }

            buttons.push(button_group);
        };

        let mut target = Vec::new();
        for btn in joltages_s.trim_start_matches('{').trim_end_matches('}').split(',') {
            target.push(btn.parse().unwrap());
        }
        
        Ok(Self {
            target,
            buttons,
        })
    }
}
