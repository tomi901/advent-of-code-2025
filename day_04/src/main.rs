use std::str::FromStr;

use anyhow::{self, Context};
use xmas::{direction::DIRECTIONS_8, display_result, map2d::ByteMap, point2d::Point2D};

fn main() -> anyhow::Result<()> {
    part_1()?;
    println!();
    part_2()?;
    Ok(())
}

fn part_1() -> anyhow::Result<()> {
    println!("Part 1:");
    let input = std::fs::read_to_string("./input.txt").context("Error reading input file.")?;
    let map = ByteMap::from_str(&input).unwrap();

    let accessible = get_accessible(&map);
    display_result(&accessible.len());
    Ok(())
}

fn part_2() -> anyhow::Result<()> {
    println!("Part 2:");

    let input = std::fs::read_to_string("./input.txt").context("Error reading input file.")?;
    let mut map = ByteMap::from_str(&input).unwrap();

    let mut count = 0;
    loop {
        let accessible = get_accessible(&map);
        if accessible.len() == 0 {
            break;
        }

        count += accessible.len();
        for point in accessible {
            map.set_tile(point, b'.');
        }
    }

    display_result(&count);
    Ok(())
}

fn get_accessible(map: &ByteMap) -> Vec<Point2D> {
    const ROLL: u8 = b'@';

    let mut result = Vec::new();
    for (point, _) in map.iter_with_points().filter(|x| *x.1 == ROLL) {
        let roll_count = DIRECTIONS_8
            .map(|d| point + d)
            .iter()
            .filter(|&p| map.get_tile(*p).is_some_and(|&t| t == ROLL))
            .count();

        if roll_count < 4 {
            result.push(point);
        }
    }

    result
}
