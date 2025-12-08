use crate::union_find::UnionFind;
use anyhow::Result;
use clap::Parser;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

mod union_find;

#[derive(Parser, Debug)]
#[command(version, long_about = "Advent of Code 2025 - Day 8")]
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

type Point = [i64; 3];

#[inline(always)]
fn euclidian_distance_squared(a: &Point, b: &Point) -> i64 {
    a.iter()
        .zip(b)
        .fold(0, |acc, (d1, d2)| acc + (d1 - d2).pow(2))
}

fn solve<R: BufRead>(reader: R, connections: usize) -> Result<(i32, i64)> {
    let junctions: Vec<Point> = reader
        .lines()
        .map_while(|res| match res {
            Ok(line) => {
                let [x, y, z] = line
                    .split(",")
                    .flat_map(|num| num.parse())
                    .collect::<Vec<i64>>()[..]
                    .try_into()
                    .ok()?;
                Some([x, y, z])
            }
            Err(_) => None,
        })
        .collect();
    let mut closest_points: BinaryHeap<Reverse<(i64, usize, usize)>> = BinaryHeap::new();
    for i in 0..junctions.len() {
        for j in i + 1..junctions.len() {
            closest_points.push(Reverse((
                euclidian_distance_squared(&junctions[i], &junctions[j]),
                i,
                j,
            )));
        }
    }
    let mut dsu = UnionFind::new(junctions.len());
    let mut connections_made: usize = 0;
    let mut task1: i32 = 0;
    let mut task2: i64 = 0;
    while connections_made < connections || task2 == 0 {
        let (_, i, j) = match closest_points.pop() {
            Some(value) => value.0,
            None => break,
        };
        dsu.union(i, j);
        connections_made += 1;
        if connections_made == connections {
            let mut sizes: Vec<&i32> = dsu.size().iter().collect();
            sizes.sort();
            task1 = sizes.iter().rev().take(3).fold(1, |acc, &b| acc * b);
        }
        if dsu.num_sets == 1 && task2 == 0 {
            task2 = junctions[i][0] * junctions[j][0];
        }
    }
    Ok((task1, task2))
}

fn main() -> Result<()> {
    let opts = Options::parse();
    let file = File::open(opts.input_file)?;
    let reader = BufReader::new(&file);
    let (task1, task2) = solve(reader, 1000)?;
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
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
",
        },
        expected = { (40, 25272) },
    )]
    fn test_solve(test_case: &str, expected: (i32, i64)) -> Result<()> {
        let (expected_task1, expected_task2) = expected;
        let input = Cursor::new(test_case.trim());
        let (task1, task2) = solve(input, 10)?;
        assert_eq!(expected_task1, task1, "Task 1");
        assert_eq!(expected_task2, task2, "Task 2");
        Ok(())
    }
}
