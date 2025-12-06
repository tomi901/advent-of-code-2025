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
    let input = std::fs::read_to_string("./test.txt").context("Error reading input file.")?;

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

    Ok(())
}

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
