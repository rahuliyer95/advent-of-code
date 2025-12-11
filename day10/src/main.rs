use anyhow::Result;
use clap::Parser;
use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, long_about = "Advent of Code 2025 - Day 10")]
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
    let mut task1 = 0;
    let task2 = 0;
    let mut expected_configs: Vec<u16> = vec![];
    let mut button_configs: Vec<Vec<Vec<u32>>> = vec![];
    let mut expected_joltages: Vec<Vec<u32>> = vec![];
    reader.lines().map_while(Result::ok).for_each(|ref line| {
        let mut chars = line.chars().peekable();
        chars.by_ref().next(); // skip '['
        expected_configs.push(
            chars
                .by_ref()
                .take_while(|&c| c != ']')
                .enumerate()
                .fold(0, |acc, (i, c)| acc | (if c == '#' { 1 } else { 0 }) << i),
        );
        chars.by_ref().next(); // skip ' '
        button_configs.push(vec![]);
        while chars.peek().is_some_and(|&c| c != '{')
            && let Some(last) = button_configs.last_mut()
        {
            chars.by_ref().next(); // skip '('
            last.push(
                chars
                    .by_ref()
                    .take_while(|&c| c != ')')
                    .flat_map(|c| c.to_digit(10))
                    .collect(),
            );
            chars.by_ref().next(); // skip ' '
        }
        expected_joltages.push(vec![]);
        while chars.peek().is_some()
            && let Some(last) = expected_joltages.last_mut()
        {
            last.push(
                chars
                    .by_ref()
                    .take_while(|&c| c != ',' && c != '}')
                    .flat_map(|c| c.to_digit(10))
                    .fold(0, |acc, digit| acc * 10 + digit),
            );
        }
    });
    let mut q: VecDeque<(u16, i64)> = VecDeque::new();
    let mut visited: HashSet<u16> = HashSet::new();
    for (expected_config, button_config) in expected_configs.iter().zip(&button_configs) {
        q.clear();
        q.push_back((0, 0));
        visited.clear();
        while let Some((state, steps)) = q.pop_front() {
            if state == *expected_config {
                task1 += steps;
                break;
            }
            for buttons in button_config.iter() {
                let new_state = buttons
                    .iter()
                    .fold(state, |acc, button| acc ^ (1 << button));
                if visited.insert(new_state) {
                    q.push_back((new_state, steps + 1));
                }
            }
        }
    }
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
        test_case = { "
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
"
        },
        expected = { (7, 33) },
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
