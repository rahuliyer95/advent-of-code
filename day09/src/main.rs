use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, long_about = "Advent of Code 2025 - Day 9")]
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

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
struct AxisAlignedRectangle {
    top_left: Point,
    bottom_right: Point,
}

impl AxisAlignedRectangle {
    fn new(a: &Point, b: &Point) -> Self {
        Self {
            top_left: Point {
                x: a.x.min(b.x),
                y: a.y.min(b.y),
            },
            bottom_right: Point {
                x: a.x.max(b.x),
                y: a.y.max(b.y),
            },
        }
    }

    #[inline(always)]
    fn area(&self) -> i64 {
        ((self.top_left.x - self.bottom_right.x).abs() as i64 + 1)
            * ((self.top_left.y - self.bottom_right.y).abs() as i64 + 1)
    }

    #[inline(always)]
    fn intersects(&self, other: &Self) -> bool {
        !(self.top_left.x > other.bottom_right.x
            || self.top_left.y > other.bottom_right.y
            || other.top_left.x > self.bottom_right.x
            || other.top_left.y > self.bottom_right.y)
    }
}

fn solve<R: BufRead>(reader: R) -> Result<(i64, i64)> {
    let points: Vec<Point> = reader
        .lines()
        .map_while(|res| match res {
            Ok(line) => line
                .split_once(",")
                .and_then(|(x, y)| x.parse::<i32>().ok().zip(y.parse::<i32>().ok()))
                .map(Point::new),
            Err(_) => None,
        })
        .collect();
    let rectangles: Vec<AxisAlignedRectangle> = (0..points.len())
        .map(|i| AxisAlignedRectangle::new(&points[i], &points[(i + 1) % points.len()]))
        .collect();
    let mut task1: i64 = 0;
    let mut task2: i64 = 0;
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let rect = AxisAlignedRectangle::new(&points[i], &points[j]);
            let area = rect.area();
            // We need to determine if the rectangle formed by points i & j does not extend the
            // polygon formed by the points in our input. To simplify the boundedness check we can
            // create a smaller rectangle that is 1 unit inside the rectangle formed by points
            // i & j. If we can ensure that this inner rectangle is strictly within bounds of the
            // polygon i.e there is no rectangle in the polygon that intersects with this inner
            // rectangle we can safely assume that the rectangle formed by points i & j is either
            // inside the polygon or shares one or more edges with the polygon but will never be
            // out of bounds. If the inner rectangle satisfies the above conditions we can use the
            // area of the rectangle formed by points i & j for our task 2 calculation.
            let inner_rect = AxisAlignedRectangle::new(
                &Point {
                    x: rect.top_left.x + 1,
                    y: rect.top_left.y + 1,
                },
                &Point {
                    x: rect.bottom_right.x - 1,
                    y: rect.bottom_right.y - 1,
                },
            );
            task1 = task1.max(area);
            if rectangles.iter().all(|r| !inner_rect.intersects(r)) {
                task2 = task2.max(area);
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
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
",
        },
        expected = { (50, 24) },
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
