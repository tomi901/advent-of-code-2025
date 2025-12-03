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

    let mut result = 0;
    for line in input.lines() {
        let biggest = get_biggest_joltage(&line);
        // println!("{line}: {biggest}");
        result += biggest;
    }

    display_result(&result);
    Ok(())
}

fn part_2() -> anyhow::Result<()> {
    println!("Part 2:");

    Ok(())
}

fn get_biggest_joltage(input: &str) -> u64 {

    let mut biggest_found: Option<(usize, u64)> = None;
    for (i, ch) in input[..input.len() - 1].chars().enumerate() {
        let digit = ch.to_digit(10).unwrap() as u64;

        if biggest_found.is_none_or(|n| n.1 < digit) {
            biggest_found = Some((i, digit))
        }
    }

    let (biggest_i, biggest_digit) = biggest_found.expect("No digit found");
    let biggest_second = input[(biggest_i + 1)..]
        .chars()
        .map(|ch| ch.to_digit(10).unwrap() as u64)
        .max()
        .unwrap();

    biggest_digit * 10 + biggest_second
}
