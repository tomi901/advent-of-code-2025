use anyhow::{self, Context};
use xmas::{display_result, point2d::Point2D};

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
