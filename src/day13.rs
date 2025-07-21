//! Day 13: Shuttle Search
//!
//! ## Problem Description
//!
//! Part 1: Find the earliest bus you can take to the airport and calculate the product
//! of its ID and the minutes you need to wait.
//!
//! Part 2: Find the earliest timestamp where each bus departs at a specific offset
//! from the timestamp (bus at index i departs at timestamp + i).
//!
//! ## Solution Approach
//!
//! **Input Parsing**: Splits input into:
//! - Earliest departure time (first line)
//! - List of bus IDs with 'x' values replaced by 0 (second line, comma-separated)
//!
//! **Part 1 Strategy**: Modular arithmetic
//! - For each bus ID, calculate wait time: (ID - (earliest % ID)) % ID
//! - Find bus with minimum wait time
//! - Return bus ID × wait time
//!
//! **Part 2 Strategy**: Chinese Remainder Theorem via incremental approach
//! - Uses iterative method to solve system of congruences
//! - At each step, finds timestamp satisfying all constraints up to current bus
//! - Uses LCM (step *= id) to maintain valid solutions across iterations
//! - Efficiently finds the earliest timestamp satisfying all bus constraints
//!
//! **Mathematical Insight**: Solves t ≡ -i (mod id) for each bus at position i.

fn parse_input(input: &str) -> (usize, Vec<usize>) {
    let lines: Vec<&str> = input.trim().lines().collect();
    (
        lines[0].parse().unwrap(),
        lines[1]
            .split(',')
            .map(|s| s.parse().unwrap_or(0))
            .collect(),
    )
}

pub fn part_one(input: &str) -> usize {
    let (earliest_departure, bus_ids) = parse_input(input);
    let mut min_wait = usize::MAX;
    let mut min_id = 0;
    for id in bus_ids {
        if id == 0 {
            continue;
        }
        let wait = id - earliest_departure % id;
        if wait < min_wait {
            min_wait = wait;
            min_id = id;
        }
    }
    min_id * min_wait
}

pub fn part_two(input: &str) -> usize {
    let (_, bus_ids) = parse_input(input);

    let mut timestamp = 0;
    let mut step = 1;
    for (i, id) in bus_ids.iter().enumerate() {
        if *id == 0 {
            continue;
        }
        while (timestamp + i) % *id != 0 {
            timestamp += step;
        }
        step *= *id;
    }
    timestamp
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(13);
        assert_eq!(part_one(&input), 295);
        assert_eq!(part_two(&input), 1068781);
    }

    #[test]
    fn example_others() {
        assert_eq!(part_two("939\n17,x,13,19"), 3417);
        assert_eq!(part_two("939\n67,7,59,61"), 754018);
        assert_eq!(part_two("939\n67,x,7,59,61"), 779210);
        assert_eq!(part_two("939\n67,7,x,59,61"), 1261476);
        assert_eq!(part_two("939\n1789,37,47,1889"), 1202161486);
    }
}
