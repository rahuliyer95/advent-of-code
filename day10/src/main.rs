use anyhow::Result;
use clap::Parser;
use good_lp::{Expression, Solution, SolverModel, default_solver, variable, variables};
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
    let mut task2 = 0;
    let mut expected_configs: Vec<u16> = vec![];
    let mut button_configs: Vec<Vec<Vec<usize>>> = vec![];
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
                    .map(|num| num as usize)
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
    for ((expected_config, button_config), expected_joltage) in expected_configs
        .iter()
        .zip(&button_configs)
        .zip(&expected_joltages)
    {
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
        // Part 2 was beyond my league and used Gemini for solving this :)

        // Integer Linear Programming (ILP) Solution for the Minimum Buttons Problem
        // This code solves a variation of the Multidimensional Change-Making Problem, where the
        // goal is to find the minimum number of "operations" (tuples) required to reach a
        // specific target state.

        // 1. The Mathematical Model
        // Since the order of applying tuples does not matter, and we seek the *minimum* count,
        // this is a linear optimization problem.
        // * **Variables:** Let $x_j$ be a non-negative integer variable representing the number of
        // times we use the $j$-th available tuple. ($x_j \in \mathbb{Z}_{\ge 0}$)
        // * **Objective Function (MINIMIZE):**
        //     We want to minimize the total number of operations:
        //     $$\min \left( \sum_{j} x_j \right)$$
        // * **Constraints (SUBJECT TO):**
        //     For every index $i$ in the configuration (from $0$ to $N-1$), the total contribution
        //     from all chosen tuples must exactly equal the target value $b_i$.
        //     $$ \sum_{j} (A_{i,j} \cdot x_j) = b_i \quad \text{for all } i=0 \dots N-1 $$
        //     Where:
        //     * $A_{i,j}$ is $1$ if tuple $j$ increments index $i$, and $0$ otherwise.
        //     * $b_i$ is the target value for index $i$ (`target_config[i]`).
        // ### 2. Implementation with `good_lp`
        // The `good_lp` crate translates this model into a format understood by the CBC solver.
        // 1.  **`variable().integer().min(0)`:** This is crucial. It enforces the constraint that
        // the number of tuples used ($x_j$) must be a whole number (Integer Programming).
        // 2.  **`vars.minimise(objective)`:** Sets the objective function (the sum of all $x_j$).
        // 3.  **`problem.with(sum_for_index_i.eq(target_value))`:** Generates one equality
        // constraint for each index $i$ in the configuration.
        // This approach guarantees finding the absolute minimum number of tuples far more
        // efficiently than graph search algorithms (like BFS or IDA*) when the target values
        // are large.

        // 1. Define Variables
        let mut vars = variables!();
        // Create one decision variable (x_j) for each available button type.
        // Crucially, they are set to be integer and non-negative.
        let button_counts: Vec<_> = (0..button_config.len())
            .map(|_| vars.add(variable().integer().min(0)))
            .collect();
        // 2. Define Objective: Minimize the sum of all tuple counts.
        // Objective = x0 + x1 + x2 + ...
        let objective: Expression = button_counts.iter().sum();
        // 3. Initialize the Problem
        let mut problem = vars.minimise(objective).using(default_solver);
        // 4. Add Constraints (Ax = b)
        // For every index 'i' in the configuration (expected_joltage)
        for (index_i, &target_value) in expected_joltage.iter().enumerate() {
            let mut sum_for_index_i = Expression::from(0);
            // Find which tuples contribute to this index
            for (button_id, affected_indices) in button_config.iter().enumerate() {
                // If the button affects index 'i', add its variable to the expression
                if affected_indices.contains(&index_i) {
                    // Contribution is 1 * x_tuple_id
                    sum_for_index_i += button_counts[button_id];
                }
            }
            // Add the constraint: Sum of contributions MUST EQUAL Target Value
            problem = problem.with(sum_for_index_i.eq(target_value));
        }
        // 5. Solve the Problem
        let solution = problem.solve()?;
        // Evaluate the objective function (total number of tuples) based on the solution.
        task2 += solution.eval(button_counts.iter().sum::<Expression>()) as i64;
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
