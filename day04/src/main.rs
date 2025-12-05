use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

const OPS: &[(isize, isize)] = &[
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[derive(Parser, Debug)]
#[command(version, long_about = "Advent of Code 2025 - Day 4")]
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
    let mut grid: Vec<Vec<char>> = reader
        .lines()
        .map_while(|res| match res {
            Ok(line) => Some(line.chars().collect()),
            Err(_) => None,
        })
        .collect();
    let mut task1 = 0;
    let mut task2 = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] != '@' {
                continue;
            }
            let mut neighbouring_rolls: i8 = 0;
            for (di, dj) in OPS {
                let x = i as isize + di;
                let y = j as isize + dj;
                if x < 0 || y < 0 || x == grid.len() as isize || y == grid[i].len() as isize {
                    continue;
                }
                if grid[x as usize][y as usize] == '@' {
                    neighbouring_rolls += 1;
                }
            }
            if neighbouring_rolls < 4 {
                task1 += 1;
            }
        }
    }
    loop {
        let mut roll_removed = false;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] != '@' {
                    continue;
                }
                let mut neighbouring_rolls: i8 = 0;
                for (di, dj) in OPS {
                    let x = i as isize + di;
                    let y = j as isize + dj;
                    if x < 0 || y < 0 || x == grid.len() as isize || y == grid[i].len() as isize {
                        continue;
                    }
                    if grid[x as usize][y as usize] == '@' {
                        neighbouring_rolls += 1;
                    }
                }
                if neighbouring_rolls < 4 {
                    task2 += 1;
                    grid[i][j] = '.';
                    roll_removed = true;
                }
            }
        }
        if !roll_removed {
            break;
        }
    }
    return Ok((task1, task2));
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
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
" },
        expected = { (13, 43) },
    )]
    fn test_solve(test_case: &str, expected: (i64, i64)) -> Result<()> {
        let (expected_task1, expected_task2) = expected;
        let input = Cursor::new(test_case.trim());
        let (task1, task2) = solve(input)?;
        assert_eq!(expected_task1, task1, "Task 1");
        assert_eq!(expected_task2, task2, "Task 2");
        Ok(())
    }
}
