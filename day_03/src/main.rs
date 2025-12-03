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

    let result: u64 = input.lines().map(|l| maximize_joltage(l, 2)).sum();
    display_result(&result);
    Ok(())
}

fn part_2() -> anyhow::Result<()> {
    println!("Part 2:");
    let input = std::fs::read_to_string("./input.txt").context("Error reading input file.")?;

    let result: u64 = input.lines().map(|l| maximize_joltage(l, 12)).sum();
    display_result(&result);
    Ok(())
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
    }

    joltage
}
