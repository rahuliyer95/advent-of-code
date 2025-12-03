use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, long_about = "Advent of Code 2025 - Day 3")]
struct Options {
    /// Path to input file
    #[arg(long, value_parser=validate_file)]
    pub input_file: PathBuf,
}

fn validate_file(file: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(file);
    if path.exists() && path.is_file() {
        Ok(path)
    } else {
        Err(format!("'{}' is not a valid file", file))
    }
}

#[inline(always)]
fn build(digit: &u8, max_joltage: &mut Vec<u8>, removals: &mut usize) -> () {
    while let Some(last) = max_joltage.last()
        && digit > last
        && *removals > 0
    {
        max_joltage.pop();
        *removals -= 1;
    }
    max_joltage.push(*digit);
}

#[inline(always)]
fn trim(max_joltage: &[u8], k: usize) -> i64 {
    return max_joltage
        .iter()
        .take(k)
        .fold(0, |acc, &d| acc * 10 + (d as i64));
}

fn solve<R: BufRead>(reader: R) -> Result<(i64, i64)> {
    let mut total_max_joltage_task1: i64 = 0;
    let mut total_max_joltage_task2: i64 = 0;
    let mut max_joltage_task1: Vec<u8> = Vec::with_capacity(32);
    let mut max_joltage_task2: Vec<u8> = Vec::with_capacity(32);
    reader.lines().map_while(Result::ok).for_each(|ref line| {
        max_joltage_task1.clear();
        max_joltage_task2.clear();
        let mut removals_task1 = line.len() - 2;
        let mut removals_task2 = line.len() - 12;
        line.chars().for_each(|ref char| {
            let digit = match char.to_digit(10) {
                Some(d) => d as u8,
                None => panic!("Invalid char {:?} in line {:?}", char, line),
            };
            build(&digit, &mut max_joltage_task1, &mut removals_task1);
            build(&digit, &mut max_joltage_task2, &mut removals_task2);
        });
        total_max_joltage_task1 += trim(&max_joltage_task1, 2);
        total_max_joltage_task2 += trim(&max_joltage_task2, 12);
    });
    Ok((total_max_joltage_task1, total_max_joltage_task2))
}

fn main() -> Result<()> {
    let opts = Options::parse();
    let file = File::open(opts.input_file)?;
    let reader = BufReader::new(&file);
    let (task1, task2) = solve(reader)?;
    println!("Task 1: {}", task1);
    println!("Task 2: {}", task2);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use parameterized::parameterized;
    use std::io::Cursor;

    #[parameterized(
        test_case = { "987654321111111\n811111111111119\n234234234234278\n818181911112111" },
        expected = { (357, 3121910778619) },
    )]
    fn test_solve(test_case: &str, expected: (i64, i64)) -> Result<()> {
        let (expected_task1, expected_task2) = expected;
        let input = Cursor::new(test_case);
        let (task1, task2) = solve(input)?;
        assert_eq!(expected_task1, task1, "Task 1");
        assert_eq!(expected_task2, task2, "Task 2");
        Ok(())
    }
}
