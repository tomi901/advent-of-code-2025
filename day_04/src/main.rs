use std::str::FromStr;

use anyhow::{self, Context};
use xmas::{direction::{DIRECTIONS_8, Direction}, display_result, map2d::{ByteMap, Map2D}, point2d::Point2D};

fn main() -> anyhow::Result<()> {
    part_1()?;
    println!();
    part_2()?;
    Ok(())
}

fn part_1() -> anyhow::Result<()> {
    println!("Part 1:");
    let input = std::fs::read_to_string("./input.txt").context("Error reading input file.")?;

    let result = get_accessible_count(&input);
    display_result(&result);
    Ok(())
}

fn part_2() -> anyhow::Result<()> {
    println!("Part 2:");

    Ok(())
}

fn get_accessible_count(input: &str) -> usize {
    let map = ByteMap::from_str(input).unwrap();
    const ROLL: u8 = b'@';

    let mut count = 0;
    for (point, _) in map.iter_with_points().filter(|x| *x.1 == ROLL) {
        let roll_count = DIRECTIONS_8
            .map(|d| point + d)
            .iter()
            .filter(|&p| map.get_tile(*p).is_some_and(|&t| t == ROLL))
            .count();

        if roll_count < 4 {
            count += 1;
        }
    }

    count
}
