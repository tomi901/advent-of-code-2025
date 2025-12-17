use std::{cmp, collections::HashMap, str::FromStr};

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

// Using https://www.reddit.com/r/adventofcode/comments/1pk87hl/comment/ntp4njq/
// to solve, I kinda get how it works now
fn part_2() -> anyhow::Result<()> {
    println!("Part 2:");
    let input = std::fs::read_to_string("./input.txt").context("Error reading input file.")?;

    let machines = input
        .lines()
        .map(JoltageMachine::from_str)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();


    let mut result = 0;
    for (i, machine) in machines.iter().enumerate() {
        let partial_result = machine.find_shortest_configuration().unwrap();
        // println!("{}", partial_result);
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

type JoltageCache = HashMap<Vec<u16>, Option<u64>>;

#[derive(Debug)]
struct JoltageMachine {
    target: Vec<u16>,
    buttons: Vec<Vec<usize>>,
}

impl JoltageMachine {
    fn find_shortest_configuration(&self) -> Option<u64> {
        let mut cache = JoltageCache::new();
        self.find_shortest_configuration_cached(&self.target, &mut cache)
    }

    fn find_shortest_configuration_cached(&self, remaining: &Vec<u16>, cache: &mut JoltageCache) -> Option<u64> {
        if remaining.iter().all(|&v| v == 0) {
            return Some(0);
        } else if let Some(&cached_result) = cache.get(remaining) {
            return cached_result;
        }

        let mut total_button_presses = None;

        // println!();
        // println!("Calculating {:?}", remaining);

        let valid_button_bitflags = self.find_parity_candidates(remaining.as_slice());
        'validity_for: for valid in valid_button_bitflags {
            let mut parity_button_presses = 0;

            // println!();
            let mut new_remaining = remaining.clone();
            for btn in (0..self.buttons.len() as u64)
                .filter(|b| (1 << b) & valid != 0)
                .map(|b| &self.buttons[b as usize]) {

                // println!("{:?}", btn);
                for &b in btn {
                    if new_remaining[b] == 0 {
                        continue 'validity_for;
                    }

                    new_remaining[b] -= 1;
                }
                
                parity_button_presses += 1;
            }

            // println!("{:?}", new_remaining);

            for r in new_remaining.iter_mut() {
                *r /= 2;
            }
            // println!("{:?}", new_remaining);

            if let Some(next_presses_odd) = self.find_shortest_configuration_cached(&new_remaining, cache) {
                // println!("For {:?} = 2 * {} + {}", new_remaining, next_presses_odd, parity_button_presses);

                let calculated_button_presses = next_presses_odd * 2 + parity_button_presses;
                total_button_presses = Some(match total_button_presses {
                    Some(previous) => cmp::min(previous, calculated_button_presses),
                    None => calculated_button_presses,
                });
            }
        }

        // println!();
        // println!("Cache {:?} = {:?}", remaining, total_button_presses);
        
        cache.insert(remaining.clone(), total_button_presses);
        total_button_presses
    }

    fn find_parity_candidates(&self, target: &[u16]) -> Vec<u64> {
        let target_bitflags = {
            let mut bitflags = 0u64;
            for (i, &val) in target.iter().enumerate() {
                if val% 2 != 0 {
                    bitflags |= 1 << i;
                }
            }
            bitflags
        };

        // println!("Targeting {:?}, bitwise: {:b}", target, target_bitflags);

        // Since all these buttons toggle odd/even values, we only need to test all possible combinations
        // of 2^n to see which button combinations will give us the results we want
        let mut valid_button_bitflags = Vec::new();
        for evaluate_bitflags in 0u64..(1 << self.buttons.len()) {
            let mut current_bitflags = 0u64;
            for (i, btn) in self.buttons.iter().enumerate() {
                if (1 << i) & evaluate_bitflags == 0 {
                    continue;
                }

                for toggle in btn {
                    current_bitflags ^= 1 << toggle;
                }
            }

            if current_bitflags == target_bitflags {
                valid_button_bitflags.push(evaluate_bitflags);
            }
        }

        valid_button_bitflags
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
