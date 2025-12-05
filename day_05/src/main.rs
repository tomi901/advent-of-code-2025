use std::{cmp::{max, min}, ops::RangeInclusive};

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

    let (first_s, second_s) = input.split_once("\n\n").unwrap();
    let ranges = first_s
        .lines()
        .map(|l| parse_range(l))
        .collect::<Vec<_>>();

    let mut result = 0;
    for line in second_s.lines() {
        let value = line.parse::<u64>().unwrap();
        let contains = ranges.iter().any(|r| r.contains(&value));
        if contains {
            result += 1;
        }
    }

    display_result(&result);
    Ok(())
}

fn part_2() -> anyhow::Result<()> {
    println!("Part 2:");
    let input = std::fs::read_to_string("./input.txt").context("Error reading input file.")?;

    let (first_s, _) = input.split_once("\n\n").unwrap();

    let unmerged_ranges = first_s.lines().map(|l| parse_range(l)).collect::<Vec<_>>();

    let mut processed_ranges = unmerged_ranges.clone();
    // println!("{processed_ranges:?}");
    loop {

        let mut to_merge: Option<(usize, usize)> = None;
        'outer_for: for (i, current_candidate) in processed_ranges.iter().enumerate() {
            for other_candidate in processed_ranges[(i + 1)..].iter().enumerate() {
                let other = other_candidate.1;

                let any_overlap = current_candidate.contains(other.start())
                    || current_candidate.contains(other.end())
                    || other.contains(current_candidate.start())
                    || other.contains(current_candidate.end());
                if any_overlap {
                    let other_i = other_candidate.0 + i + 1;
                    to_merge = Some((i, other_i));
                    break 'outer_for;
                }
            }
        }

        if let Some((i, j)) = to_merge {
            let r1 = processed_ranges[i].clone();
            let r2 = processed_ranges[j].clone();

            let start = *min(r1.start(), r2.start());
            let end = *max(r1.end(), r2.end());
            let merged_range = start..=end;

            processed_ranges[i] = merged_range;
            processed_ranges.remove(j);

            // println!("Merging {r1:?} ({i}) and {r2:?} ({j}) => {processed_ranges:?}");
        } else {
            break;
        }
    }

    let mut result = 0;
    for range in processed_ranges {
        let sum = range.end() - range.start() + 1;
        // println!("{:?} = {}", range, sum);
        result += sum;
    }

    display_result(&result);
    Ok(())
}

fn parse_range(s: &str) -> RangeInclusive<u64> {
    let (first_s, second_s) = s.split_once('-').unwrap();
    let first = first_s.parse::<u64>().unwrap();
    let second = second_s.parse::<u64>().unwrap();
    first..=second
}
