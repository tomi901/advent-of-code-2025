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
    Ok(())
}

fn part_2() -> anyhow::Result<()> {
    println!("Part 2:");
    let input = std::fs::read_to_string("./input.txt").context("Error reading input file.")?;

    let mut result = 0;
    for line in input.lines() {
        let biggest = maximize_joltage(&line, 12);
        // println!("{line} => {biggest}");
        result += biggest;
    }

    display_result(&result);
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

fn maximize_joltage(input: &str, digit_count: u64) -> u64 {
    let mut joltage = 0;
    let mut min_i = 0;

    for current_digit_n in 1..=digit_count {
        let max_i = input.len() - (digit_count - current_digit_n) as usize;

        let mut found: Option<(usize, u64)> = None;
        for (i, ch) in input[min_i..max_i].char_indices() {
            let digit = ch.to_digit(10).unwrap() as u64;

            if found.is_none_or(|f| f.1 < digit) {
                found = Some((i, digit));
            }
        }

        let (found_i, found_ch) = found.expect("input too short");

        joltage *= 10;
        joltage += found_ch;

        min_i += found_i + 1;
        // dbg!(found_i, found_ch);
        // println!("Repositioned at {min_i} ({})", input.chars().nth(min_i).unwrap_or(' '));
    }

    joltage
}
