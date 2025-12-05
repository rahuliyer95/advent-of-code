use anyhow::Result;
use clap::Parser;
use std::cmp::{max, min};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, long_about = "Advent of Code 2025 - Day 5")]
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
    let mut lines = reader.lines();
    let mut fresh_ingredients: Vec<(i64, i64)> = lines
        .by_ref()
        .map_while(|result| match result {
            Ok(line) if !line.is_empty() => line
                .split_once("-")
                .and_then(|(start, end)| start.parse().ok().zip(end.parse().ok())),
            _ => None,
        })
        .collect();
    let queries: Vec<i64> = lines
        .map_while(Result::ok)
        .flat_map(|ref line| line.parse())
        .collect();
    // Merge intervals
    fresh_ingredients.sort();
    let mut i = 1;
    while i < fresh_ingredients.len() {
        if fresh_ingredients[i].0 > fresh_ingredients[i - 1].1 {
            i += 1;
            continue;
        }
        fresh_ingredients[i - 1] = (
            min(fresh_ingredients[i - 1].0, fresh_ingredients[i].0),
            max(fresh_ingredients[i - 1].1, fresh_ingredients[i].1),
        );
        fresh_ingredients.remove(i);
    }
    let mut task1: i64 = 0;
    for query in &queries {
        if let Some(_) = fresh_ingredients
            .iter()
            .find(|(start, end)| start <= query && query <= end)
        {
            task1 += 1;
        }
    }
    let task2 = fresh_ingredients
        .iter()
        .flat_map(|(start, end)| *start..=*end)
        .count() as i64;
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
        test_case = { "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32" },
        expected = { (3, 14) },
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
