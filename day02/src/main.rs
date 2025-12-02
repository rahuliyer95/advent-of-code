use anyhow::Result;
use clap::Parser;
use std::fmt::Write;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, long_about = "Advent of Code 2025 - Day 2")]
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

fn solve<R: BufRead>(mut reader: R) -> Result<(i64, i64)> {
    let mut buf: Vec<u8> = vec![];
    let mut task1: i64 = 0;
    let mut task2: i64 = 0;
    let mut number_concat = String::with_capacity(40);
    while reader.read_until(',' as u8, &mut buf)? != 0 {
        let mut iter = buf.iter_mut();
        let low = iter
            .by_ref()
            .take_while(|c| **c != '-' as u8)
            .fold(0, |acc, c| acc * 10 + (*c - 48) as i64);
        let high = iter
            .by_ref()
            .flat_map(|c| (*c as char).to_digit(10))
            .fold(0, |a, c| a * 10 + c as i64);
        for number in low..=high {
            let number_string = number.to_string();
            let mid = number_string.len() / 2;
            if number_string.len() % 2 == 0 && number_string[0..mid] == number_string[mid..] {
                task1 += number;
                task2 += number;
            } else {
                // https://leetcode.com/problems/repeated-substring-pattern/description/
                number_concat.clear();
                write!(&mut number_concat, "{}{}", number, number)?;
                if let Some(_) = number_concat[1..(number_concat.len() - 1)].find(&number_string) {
                    task2 += number;
                }
            }
        }
        buf.clear();
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
        test_case = {
            "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
        },
        expected = { (1227775554, 4174379265) },
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
