use std::{collections::HashSet, str::FromStr};

use anyhow::{self, Context};
use xmas::{display_result, map2d::ByteMap, point2d::Point2D};

fn main() -> anyhow::Result<()> {
    part_1()?;
    println!();
    part_2()?;
    Ok(())
}

fn part_1() -> anyhow::Result<()> {
    println!("Part 1:");
    let input = std::fs::read_to_string("./input.txt").context("Error reading input file.")?;

    let mut map = ByteMap::from_str(&input)?;
    let start = map.find(&b'S').unwrap();

    let mut path = HashSet::new();
    let result: u64 = get_split_amount(&map, start, &mut path);

    for point in path {
        map.set_tile(point, b'|');
    }

    // println!("{}", map);

    display_result(&result);
    Ok(())
}

fn part_2() -> anyhow::Result<()> {
    println!("Part 2:");

    Ok(())
}

fn get_split_amount(map: &ByteMap, from: Point2D, path: &mut HashSet<Point2D>) -> u64 {
    if from.0 < 0 || from.0 as usize >= map.width() {
        return 0;
    }

    for y in from.1..(map.height() as isize) {
        let point = Point2D(from.0, y);
        if path.contains(&point) {
            return 0;
        }

        if !map.is_inside(point) {
            continue;
        }

        if !map.get_tile(point).is_some_and(|&t| t == b'^') {
            path.insert(point);
            continue;
        }

        // let mut map_copy = map.clone();
        // for &p in path.iter() {
        //     map_copy.set_tile(p, b'|');
        // }
        // println!("{}", map_copy);

        let left_splits = get_split_amount(map, Point2D(from.0 - 1, y), path);
        let right_splits = get_split_amount(map, Point2D(from.0 + 1, y), path);
        return left_splits + right_splits + 1;
    }

    0
}
