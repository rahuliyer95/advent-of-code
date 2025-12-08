use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, long_about = "Advent of Code 2025 - Day 7")]
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

fn solve<R: BufRead>(reader: R) -> Result<(i32, i64)> {
    let mut grid: Vec<Vec<char>> = reader
        .lines()
        .map_while(|res| match res {
            Ok(line) => Some(line.chars().collect()),
            Err(_) => None,
        })
        .collect();
    let mut splits: Vec<Vec<i64>> = grid
        .iter()
        .map(|row| row.iter().map(|c| if *c == 'S' { 1 } else { 0 }).collect())
        .collect();
    let mut num_splits = 0;
    for i in 1..grid.len() {
        for j in 0..grid[i].len() {
            splits[i][j] = splits[i - 1][j];
            if grid[i][j] == '.' && (grid[i - 1][j] == 'S' || grid[i - 1][j] == '|') {
                grid[i][j] = '|';
            }
        }
        for j in 0..grid[i].len() {
            if grid[i][j] != '^' {
                continue;
            }
            if grid[i - 1][j] == '|' {
                num_splits += 1;
            }
            if j > 0 {
                grid[i][j - 1] = '|';
                splits[i][j - 1] += splits[i][j];
            }
            if j + 1 < grid[0].len() {
                grid[i][j + 1] = '|';
                splits[i][j + 1] += splits[i][j];
            }
            splits[i][j] = 0;
        }
    }
    Ok((
        num_splits,
        splits
            .last()
            .and_then(|row| Some(row.iter().sum()))
            .unwrap_or(0),
    ))
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
        test_case = { "
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
",
        },
        expected = { (21, 40) },
    )]
    fn test_solve(test_case: &str, expected: (i32, i64)) -> Result<()> {
        let (expected_task1, expected_task2) = expected;
        let input = Cursor::new(test_case.trim());
        let (task1, task2) = solve(input)?;
        assert_eq!(expected_task1, task1, "Task 1");
        assert_eq!(expected_task2, task2, "Task 2");
        Ok(())
    }
}
