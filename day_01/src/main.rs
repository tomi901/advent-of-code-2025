use anyhow::{self, Context};
use day_01::{get_new_password_from_input, get_password_from_input};
use xmas::display_result;

fn main() -> anyhow::Result<()> {
    test()?;
    println!();
    // part_1()?;
    println!();
    part_2()?;
    Ok(())
}

fn test() -> anyhow::Result<()> {
    println!("Test:");
    let input = std::fs::read_to_string("./test.txt").context("Error reading input file.")?;

    let result = get_new_password_from_input(&input, 50, 100);

    display_result(&result);
    Ok(())
}

fn part_1() -> anyhow::Result<()> {
    println!("Part 1:");
    let input = std::fs::read_to_string("./input.txt").context("Error reading input file.")?;

    let result = get_password_from_input(&input, 50, 100);

    display_result(&result);
    Ok(())
}

fn part_2() -> anyhow::Result<()> {
    println!("Part 2:");
    let input = std::fs::read_to_string("./input.txt").context("Error reading input file.")?;

    let result = get_new_password_from_input(&input, 50, 100);

    display_result(&result);
    Ok(())
}
