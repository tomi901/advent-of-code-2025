use std::{collections::{HashMap, HashSet}, str::FromStr};

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
    let input = std::fs::read_to_string("./input.txt").context("Error reading input file.")?;

    let map = ByteMap::from_str(&input)?;
    let result: u64 = get_timeline_amount(&map);

    display_result(&result);
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

fn get_timeline_amount(map: &ByteMap) -> u64 {
    let mut cache = HashMap::new();
    let start = map.find(&b'S').unwrap();
    return get_cached_timelines(map, start, &mut cache)
}

fn get_cached_timelines(map: &ByteMap, from: Point2D, cache: &mut HashMap<Point2D, u64>) -> u64 {
    for y in from.1..(map.height() as isize) {
        let point = Point2D(from.0, y);
        if !map.is_inside(point) {
            break;
        }

        if let Some(&cached_timelines) = cache.get(&point) {
            return cached_timelines;
        }

        if !map.get_tile(point).is_some_and(|&t| t == b'^') {
            continue;
        }

        let left_timelines = get_cached_timelines(map, Point2D(from.0 - 1, y), cache);
        let right_timelines = get_cached_timelines(map, Point2D(from.0 + 1, y), cache);
        let timelines = left_timelines + right_timelines;

        cache.insert(from, timelines);
        return timelines;
    }

    1
}
