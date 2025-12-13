use std::{cmp, collections::HashMap};
use anyhow::{self, Context};
use itertools::Itertools;
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
    let red_tiles = input
        .lines()
        .map(|l| l.parse::<Point2D>())
        .collect::<Result<Vec<_>, _>>()?;

    let result = find_largest_area(&red_tiles).unwrap();

    display_result(&result);
    Ok(())
}

fn part_2() -> anyhow::Result<()> {
    println!("Part 2:");
    let input = std::fs::read_to_string("./input.txt").context("Error reading input file.")?;
    let red_tiles = input
        .lines()
        .map(|l| l.parse::<Point2D>())
        .collect::<Result<Vec<_>, _>>()?;

    let result = find_largest_area_enclosed(&red_tiles).unwrap();

    display_result(&result);
    Ok(())
}

fn find_largest_area(tiles: &[Point2D]) -> Option<usize> {
    let mut largest = None;
    for (i, from) in tiles.iter().enumerate() {
        for to in &tiles[(i + 1)..] {
            let width = from.0.abs_diff(to.0) + 1;
            let height = from.1.abs_diff(to.1) + 1;
            let area = width * height;

            if largest.is_none_or(|l| l < area) {
                largest = Some(area);
            }
        }
    }

    largest
}

fn find_largest_area_enclosed(red_tiles: &[Point2D]) -> Option<usize> {
    // TODO?:
    // Input numbers are pretty big, so we probably must avoid brute force

    let x_lookup = create_lookup(red_tiles.iter().map(|p| p.0));
    let y_lookup = create_lookup(red_tiles.iter().map(|p| p.1));

    let mut map = ByteMap::new_filled(Point2D(x_lookup.len() as isize, y_lookup.len() as isize), b'.');
    let compressed_points = red_tiles
        .iter()
        .map(|p| Point2D(x_lookup[&p.0], y_lookup[&p.1]))
        .collect::<Vec<_>>();

    for (i, &point) in compressed_points.iter().enumerate() {
        let next_point = compressed_points[(i + 1) % compressed_points.len()];
        let diff = next_point - point;

        match diff {
            Point2D(0, _) => {
                let from = cmp::min(next_point.1, point.1);
                let to = cmp::max(next_point.1, point.1);
                for y in from..=to {
                    map.set_tile(Point2D(point.0, y), b'#');
                }
            },
            Point2D(_, 0) => {
                let from = cmp::min(next_point.0, point.0);
                let to = cmp::max(next_point.0, point.0);
                for x in from..=to {
                    map.set_tile(Point2D(x, point.1), b'#');
                }
            },
            _ => unreachable!()
        }
    }

    for y in 0..map.height() {
        let mut inside_line = false;
        let mut hits = 0;
        for x in 0..map.width() {
            let previous = map.get_tile(Point2D(x as isize - 1, y as isize)).cloned().unwrap_or(b'.');
            let current_point = Point2D(x as isize, y as isize);
            let current = *map.get_tile(current_point).unwrap();

            if !inside_line && previous == b'.' && current == b'#' {
                inside_line = true;
            } else if inside_line && previous == b'#' && current == b'.' {
                inside_line = false;
                hits += 1;
            }

            if hits % 2 != 0 {
                map.set_tile(current_point, b'#');
            }
        }
    }

    // println!("{}", &map);

    let mut largest = None;
    'outer: for (i, &point) in compressed_points.iter().enumerate() {
        for (j, &other_point) in compressed_points[(i + 1)..].iter().enumerate() {
            let from_x = cmp::min(point.0, other_point.0);
            let to_x = cmp::max(point.0, other_point.0);
            let from_y = cmp::min(point.1, other_point.1);
            let to_y = cmp::max(point.1, other_point.1);

            let all_filled = (from_x..=to_x)
                .flat_map(|x| (from_y..=to_y).map(move |y| Point2D(x, y)))
                .all(|p| map.get_tile(p).is_some_and(|&t| t == b'#'));
            if !all_filled {
                // println!("SKIP");
                continue 'outer;
            }

            let actual_point = red_tiles[i];
            let actual_other_point = red_tiles[i + 1 + j];
            let width = actual_point.0.abs_diff(actual_other_point.0) + 1;
            let height = actual_point.1.abs_diff(actual_other_point.1) + 1;
            let area = width * height;

            if largest.is_none_or(|l| l < area) {
                largest = Some(area);
            }
        }
    }

    largest
}

fn create_lookup(values: impl Iterator<Item = isize>) -> HashMap<isize, isize> {
    let mut compressed_values = values.unique().collect::<Vec<_>>();
    compressed_values.sort();
    compressed_values
        .iter()
        .enumerate()
        .map(|x| (*x.1, x.0 as isize))
        .collect()
}
