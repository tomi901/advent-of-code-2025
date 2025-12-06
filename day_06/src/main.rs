use std::{fmt::Debug, str::FromStr};

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

    let mut problems = Vec::new();
    for line in input.lines() {
        
        for (i, s) in line.split_whitespace().enumerate() {
            if i >= problems.len() {
                problems.push(Problem::new());
            }

            let problem = &mut problems[i];
            match s {
                "*" => { problem.operator = Operator::Mul },
                "+" => { problem.operator = Operator::Add },
                _ => {
                    let num = s.parse::<i64>().unwrap();
                    problem.add_num(num);
                }
            }
        }
    }

    let result: i64 = problems.iter().map(|p| p.resolve()).sum();
    display_result(&result);
    Ok(())
}

fn part_2() -> anyhow::Result<()> {
    println!("Part 2:");
    let input = std::fs::read_to_string("./input.txt").context("Error reading input file.")?;

    let mut problems = Vec::new();

    let map = ByteMap::from_str(&input)?;
    let width = map.width() as isize;
    let last_y = (map.height() - 1) as isize;

    let mut x_start;
    let mut x_end = 0;
    loop {
        x_start = x_end;
        if x_start >= width {
            break;
        }

        x_end = x_start + 1;

        // println!("{}", Point2D(x_start, last_y));
        let operator = match &map.get_tile(Point2D(x_start, last_y)).unwrap() {
            b'*' => Operator::Mul,
            b'+' => Operator::Add,
            _ => unimplemented!("invalid operator")
        };

        while map.get_tile(Point2D(x_end, last_y)) == Some(&b' ') {
            x_end += 1;
        }

        // This fixes the last column being ignored
        if x_end >= width {
            x_end = width + 1;
        }

        let mut problem = Problem::new();
        problem.operator = operator;

        for column in x_start..(x_end - 1) {
            let mut num = 0;

            for y in 0..last_y {
                let b = *map.get_tile(Point2D(column, y)).unwrap();
                let digit = match b {
                    b' ' => continue,
                    b'0'..=b'9' => (b - b'0') as i64,
                    _ => unimplemented!()
                };

                num *= 10;
                num += digit;
            }

            problem.add_num(num);
        }

        // dbg!(&problem);
        problems.push(problem);
    }

    let result: i64 = problems.iter().map(|p| p.resolve()).sum();
    display_result(&result);
    Ok(())
}

#[derive(Debug)]
struct Problem {
    pub operator: Operator,
    nums: Vec<i64>,
}

impl Problem {
    pub fn new() -> Self {
        Self { operator: Operator::Add, nums: Vec::new() }
    }

    pub fn add_num(&mut self, n: i64) {
        self.nums.push(n);
    }

    pub fn resolve(&self) -> i64 {
        match self.operator {
            Operator::Add => self.nums.iter().sum(),
            Operator::Mul => self.nums.iter().cloned().reduce(|a, b| a * b).unwrap(),
        }
    }
}

enum Operator {
    Add,
    Mul,
}

impl Debug for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Add => write!(f, "+"),
            Self::Mul => write!(f, "*"),
        }
    }
}
