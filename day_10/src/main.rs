use std::str::FromStr;

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
    let input = std::fs::read_to_string("./test.txt").context("Error reading input file.")?;

    let machines = input
        .lines()
        .map(Machine::from_str)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    println!("{:?}", machines);

    // display_result(&result);
    Ok(())
}

fn part_2() -> anyhow::Result<()> {
    println!("Part 2:");

    Ok(())
}

#[derive(Debug)]
struct Machine {
    target: u64,
    buttons: Vec<u64>,
    joltages: Vec<u64>,
}

impl FromStr for Machine {
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
        let joltages_s = loop {
            let section = split.next();
            if section.is_none() {
                panic!("no joltages");
            }

            let btns = section.unwrap();
            if btns.starts_with('{') {
                break btns;
            }

            let mut button_group = 0;
            for btn in btns.trim_start_matches('(').trim_end_matches(')').split(',') {
                // println!("{}", btn);
                button_group |= 1 << btn.parse::<u64>().unwrap();
            }

            buttons.push(button_group);
        };

        let mut joltages = Vec::new();
        for btn in joltages_s.trim_start_matches('{').trim_end_matches('}').split(',') {
            joltages.push(btn.parse().unwrap());
        }
        
        Ok(Machine {
            target,
            buttons,
            joltages,
        })
    }
}
