//! Day 5: Binary Boarding
//!
//! ## Problem Description
//!
//! Part 1: Find the highest seat ID on a boarding pass using binary space partitioning.
//! Part 2: Find your seat ID which is missing from the list but has adjacent seats.
//!
//! ## Solution Approach
//!
//! **Input Parsing**: Reads each line as a boarding pass string (e.g., "FBFBBFFRLR").
//!
//! **Binary Space Partitioning**: The `decode` function uses binary search:
//! - 'F'/'L' = take lower half (reduce upper bound)
//! - 'B'/'R' = take upper half (increase lower bound)
//! - First 7 chars: row (0-127), Last 3 chars: column (0-7)
//! - Seat ID = row * 8 + column
//!
//! **Part 1 Strategy**: Find maximum seat ID
//! - Decode all boarding passes to seat IDs
//! - Return the maximum value
//!
//! **Part 2 Strategy**: Find missing seat ID
//! - Decode all boarding passes and sort seat IDs
//! - Find gap where seat ID+1 doesn't equal next seat ID
//! - Return the missing seat ID (your seat)
//!
//! **Binary Search Logic**: Uses half-interval search to efficiently determine
//! row/column from boarding pass characters.

fn parse_input(input: &str) -> Vec<&str> {
    input.trim().lines().collect()
}

fn decode(s: &str) -> u16 {
    let (mut a, mut b) = (0, (1 << s.len()) - 1);
    for ch in s.trim().chars() {
        #[allow(clippy::manual_div_ceil)]
        let delta = (b - a + 1) / 2;
        match ch {
            'F' | 'L' => b -= delta,
            'B' | 'R' => a += delta,
            _ => panic!("unknown char: {ch}"),
        }
    }
    assert_eq!(a, b);
    a
}

pub fn part_one(input: &str) -> u16 {
    parse_input(input)
        .iter()
        .map(|s| (decode(&s[..7]), decode(&s[s.len() - 3..])))
        .map(|(a, b)| a * 8 + b)
        .max()
        .unwrap()
}

pub fn part_two(input: &str) -> u16 {
    let mut seats = parse_input(input)
        .iter()
        .map(|s| (decode(&s[..7]), decode(&s[s.len() - 3..])))
        .map(|(a, b)| a * 8 + b)
        .collect::<Vec<_>>();
    seats.sort_unstable();
    seats
        .windows(2)
        .find(|it| it[0] + 1 != it[1])
        .map(|it| it[0] + 1)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(5);
        assert_eq!(part_one(&input), 820);
    }
}
