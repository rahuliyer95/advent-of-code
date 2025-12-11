use anyhow::Result;
use clap::Parser;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, long_about = "Advent of Code 2025 - Day 11")]
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

fn count_paths(graph: &HashMap<String, Vec<String>>, source: &str, destination: &str) -> i64 {
    let mut stack: VecDeque<&str> = VecDeque::new();
    let mut visited: HashSet<&str> = HashSet::new();
    let mut num_paths_from_node: HashMap<&str, i64> = HashMap::from([(destination, 1)]);
    if graph.contains_key(source) {
        stack.push_back(source);
    }
    while let Some(&u) = stack.back() {
        if num_paths_from_node.contains_key(u) {
            stack.pop_back();
            continue;
        }
        if visited.insert(u) {
            graph
                .get(u)
                .iter()
                .flat_map(|v| v.iter().map(String::as_str))
                .filter(|&v| !num_paths_from_node.contains_key(v))
                .for_each(|v| stack.push_back(v));
        } else {
            num_paths_from_node.insert(
                u,
                graph
                    .get(u)
                    .iter()
                    .flat_map(|v| v.iter().map(String::as_str))
                    .flat_map(|v| num_paths_from_node.get(v))
                    .copied()
                    .sum(),
            );
            stack.pop_back();
        }
    }
    num_paths_from_node.get(source).copied().unwrap_or(0)
}

fn solve<R: BufRead>(reader: R) -> Result<(i64, i64)> {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    reader.lines().map_while(Result::ok).for_each(|ref line| {
        if let Some((source, destinations)) = line.split_once(":") {
            let source = source.to_string();
            destinations
                .split(" ")
                .filter_map(|destination| match destination.trim() {
                    destination if !destination.is_empty() => Some(destination),
                    _ => None,
                })
                .for_each(|destination| {
                    graph
                        .entry(source.to_string())
                        .and_modify(|destinations| destinations.push(destination.to_string()))
                        .or_insert(vec![destination.to_string()]);
                });
        }
    });
    let task1 = count_paths(&graph, "you", "out");
    // there are 2 ways to reach from svr -> out while visiting both dac & fft
    // 1. svr -> dac -> fft -> out
    // 2. svr -> fft -> dac -> out
    // we count both paths separately and add them.
    let task2_path1 = count_paths(&graph, "svr", "dac")
        * count_paths(&graph, "dac", "fft")
        * count_paths(&graph, "fft", "out");
    let task2_path2 = count_paths(&graph, "svr", "fft")
        * count_paths(&graph, "fft", "dac")
        * count_paths(&graph, "dac", "out");
    let task2 = task2_path1 + task2_path2;
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
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out",
            "
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out"
        },
        expected = { (5, 0), (0, 2) },
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
