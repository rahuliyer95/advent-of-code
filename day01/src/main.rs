use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, long_about = "Advent of Code 2025 - Day 1")]
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

fn solve<R: BufRead>(reader: R) -> Result<(i32, i32)> {
    let lines = reader.lines();
    let mut dial = 50;
    let mut task1 = 0;
    let mut task2 = 0;
    lines.map_while(Result::ok).for_each(|ref line| {
        let mut chars = line.chars();
        let direction = match chars.next() {
            Some('R') => 1,
            Some('L') => -1,
            char => panic!("Invalid character {:?}", char),
        };
        let distance = chars.fold(0u32, |acc, ref c| acc * 10 + c.to_digit(10).unwrap_or(0)) as i32;
        for _ in 0..distance {
            let dial_was_zero = dial == 0;
            dial = match dial + direction {
                ..0 => 99,
                100.. => 0,
                new_dial => new_dial,
            };
            if !dial_was_zero && dial == 0 {
                task2 += 1;
            }
        }
        if dial == 0 {
            task1 += 1;
        }
    });
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
        test_case = {
            "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82\n",
            "L50\nR50",
            "L50\nL50",
            "R50\nL50",
            "R50\nR50",
            "R150\nL50",
            "R150\nR50",
        },
        expected = {
            (3, 6),
            (1, 1),
            (1, 1),
            (1, 1),
            (1, 1),
            (1, 2),
            (1, 2),
        },
    )]
    fn test_solve(test_case: &str, expected: (i32, i32)) -> Result<()> {
        let (expected_task1, expected_task2) = expected;
        let input = Cursor::new(test_case);
        let (task1, task2) = solve(input)?;
        assert_eq!(expected_task1, task1, "Task 1");
        assert_eq!(expected_task2, task2, "Task 2");
        Ok(())
    }
}
