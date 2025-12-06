use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, long_about = "Advent of Code 2025 - Day 6")]
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

fn solve<R: BufRead>(reader: R) -> Result<(i64, i64)> {
    let mut grid_task1: Vec<Vec<i32>> = vec![];
    let mut grid_task2: Vec<Vec<char>> = vec![];
    let mut ops: Vec<char> = vec![];
    for line in reader.lines().map_while(Result::ok) {
        if line.starts_with("+") || line.starts_with("*") {
            line.chars()
                .filter_map(|op| match op {
                    '+' | '*' => Some(op),
                    _ => None,
                })
                .for_each(|op| ops.push(op));
            continue;
        }
        grid_task1.push(
            line.split(" ")
                .flat_map(|num| match num.trim() {
                    num if !num.is_empty() => num.parse::<i32>().ok(),
                    _ => None,
                })
                .collect(),
        );
        grid_task2.push(line.chars().collect());
    }
    let task1 = ops
        .iter()
        .enumerate()
        .map(|(col, op)| match op {
            '+' => (0..grid_task1.len()).fold(0, |acc, row| acc + grid_task1[row][col] as i64),
            '*' => (0..grid_task1.len()).fold(1, |acc, row| acc * grid_task1[row][col] as i64),
            _ => unreachable!("Unsupported operation {}", op),
        })
        .sum();
    let mut col = grid_task2[0].len() as isize - 1;
    let task2 = ops
        .iter()
        .rev()
        .map(|op| {
            let mut res: i64 = if *op == '*' { 1 } else { 0 };
            while col >= 0 {
                let number = (0..grid_task2.len())
                    .filter_map(|row| grid_task2[row][col as usize].to_digit(10))
                    .fold(0, |acc, digit| acc * 10 + digit as i64);
                if number == 0 {
                    break;
                }
                res = match op {
                    '*' => res * number,
                    '+' => res + number,
                    _ => unreachable!("Unsupported operation {}", op),
                };
                col -= 1;
            }
            col -= 1;
            res
        })
        .sum();
    Ok((task1, task2))
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
        test_case = { "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  " },
        expected = { (4277556, 3263827) },
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
