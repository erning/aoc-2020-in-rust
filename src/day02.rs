//! Day 2: Password Philosophy
//!
//! ## Problem Description
//!
//! Part 1: Validate passwords based on character count policy - each line contains
//! a range (min-max), a character, and a password. Count how many passwords
//! have the character appear between min and max times (inclusive).
//!
//! Part 2: Validate passwords based on position policy - each line contains
//! two positions (1-indexed), a character, and a password. Count how many
//! passwords have the character appear in exactly one of the two positions.
//!
//! ## Solution Approach
//!
//! **Input Parsing**: Parses each line in format "min-max char: password" into:
//! - Policy tuple: (min_position, max_position, character)
//! - Password string
//!
//! **Part 1 Strategy**: Character frequency counting
//! - For each password, count occurrences of the specified character
//! - Check if count falls within the min-max range
//! - Count valid passwords using iterator filters
//!
//! **Part 2 Strategy**: XOR position checking
//! - Check if character appears at first position (min-1 for 0-indexing)
//! - Check if character appears at second position (max-1 for 0-indexing)
//! - Valid when exactly one position contains the character (XOR logic)
//! - Count valid passwords using iterator filters
//!
//! **Parsing Notes**: Uses split on ['-', ' ', ':'] delimiters and careful indexing
//! to extract policy components and password from each line.

type Policy = (usize, usize, char);

fn parse_input(input: &str) -> Vec<(Policy, &str)> {
    input
        .trim()
        .lines()
        .map(|s| {
            //
            let parts: Vec<&str> =
                s.split(['-', ' ', ':']).map(|s| s.trim()).collect();
            (
                (
                    parts[0].parse().unwrap(),
                    parts[1].parse().unwrap(),
                    parts[2].chars().next().unwrap(),
                ),
                parts[4],
            )
        })
        .collect()
}

pub fn part_one(input: &str) -> usize {
    parse_input(input)
        .iter()
        .filter(|((lo, hi, ch), pwd)| {
            (*lo..=*hi).contains(&pwd.chars().filter(|v| v == ch).count())
        })
        .count()
}

pub fn part_two(input: &str) -> usize {
    parse_input(input)
        .iter()
        .filter(|((lo, hi, ch), pwd)| {
            (pwd.chars().nth(lo - 1) == Some(*ch))
                != (pwd.chars().nth(hi - 1) == Some(*ch))
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(2);
        assert_eq!(part_one(&input), 2);
        assert_eq!(part_two(&input), 1);
    }
}
