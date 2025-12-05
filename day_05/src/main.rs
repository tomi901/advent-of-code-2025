use std::ops::RangeInclusive;

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

    let (first_s, second_s) = input.split_once("\n\n").unwrap();
    let ranges = first_s
        .lines()
        .map(|l| parse_range(l))
        .collect::<Vec<_>>();

    let mut result = 0;
    for line in second_s.lines() {
        let value = line.parse::<u64>().unwrap();
        let contains = ranges.iter().any(|r| r.contains(&value));
        if contains {
            result += 1;
        }
    }

    display_result(&result);
    Ok(())
}

fn part_2() -> anyhow::Result<()> {
    println!("Part 2:");

    Ok(())
}

fn parse_range(s: &str) -> RangeInclusive<u64> {
    let (first_s, second_s) = s.split_once('-').unwrap();
    let first = first_s.parse::<u64>().unwrap();
    let second = second_s.parse::<u64>().unwrap();
    first..=second
}
