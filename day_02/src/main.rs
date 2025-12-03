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

    for range_input in input.trim().split(',') {
        let (from_s, to_s) = range_input.trim().split_once('-').unwrap();
        let range = from_s.parse::<u64>().unwrap()..=to_s.parse::<u64>().unwrap();

        for n in range {
            if !is_valid_half(n) {
                // println!("{}: {}", range_input, n);
                result += n;
            }
        }
    }

    display_result(&result);
    Ok(())
}

fn part_2() -> anyhow::Result<()> {
    println!("Part 2:");
    let input = std::fs::read_to_string("./input.txt").context("Error reading input file.")?;


    let mut result = 0;

    for range_input in input.trim().split(',') {
        let (from_s, to_s) = range_input.trim().split_once('-').unwrap();
        let range = from_s.parse::<u64>().unwrap()..=to_s.parse::<u64>().unwrap();

        for n in range {
            if !is_valid_any_amount(n) {
                // println!("{}: {}", range_input, n);
                result += n;
            }
        }
    }

    display_result(&result);
    Ok(())
}

fn is_valid_half(id: u64) -> bool {
    let s = id.to_string();
    let len: usize = s.len();

    if len % 2 != 0 {
        return true;
    }

    let (first_half, second_half) = s.split_at(len / 2);
    first_half != second_half
}

fn is_valid_any_amount(id: u64) -> bool {
    let s = id.to_string();
    let len: usize = s.len();

    for pattern_len in 1..=(len / 2) {
        if pattern_len == 0 {
            continue;
        }

        if len % pattern_len != 0 {
            continue;
        }

        let mut is_repeating = true;

        let pattern = &s[0..pattern_len];
        let repeats = len / pattern_len;
        for i in 1..repeats {
            let offset = i * pattern_len;
            let current_section = &s[offset..(offset + pattern_len)];
            
            if pattern != current_section {
                is_repeating = false;
                break;
            }
        }

        if is_repeating {
            return false;
        }
    }

    true
}
