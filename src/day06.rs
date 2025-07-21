//! Day 6: Custom Customs
//!
//! ## Problem Description
//!
//! Part 1: Count the total number of unique questions answered "yes" across all groups.
//! Part 2: Count the total number of questions answered "yes" by everyone in each group.
//!
//! ## Solution Approach
//!
//! **Input Parsing**: Splits input by double newlines to separate groups,
//! then converts each person's answers into byte slices for efficient processing.
//!
//! **Part 1 Strategy**: Union of answers per group
//! - For each group, track which letters (a-z) appear in any person's answers
//! - Uses boolean vector as a simple set representation
//! - Counts unique questions per group and sums across all groups
//!
//! **Part 2 Strategy**: Intersection of answers per group
//! - For each group, count how many people answered each question
//! - Questions are valid only if count equals group size
//! - Uses integer vector to track frequency of each letter
//! - Counts questions answered by everyone per group and sums across all groups
//!
//! **Efficiency**: Uses byte arithmetic (ch - b'a') for O(1) character indexing,
//! avoiding string allocations and leveraging contiguous memory access.

fn parse_input(input: &str) -> Vec<Vec<&[u8]>> {
    input
        .trim()
        .split("\n\n")
        .map(|section| section.trim().lines().map(|s| s.as_bytes()).collect())
        .collect()
}

pub fn part_one(input: &str) -> usize {
    parse_input(input)
        .iter()
        .map(|grid| {
            let mut m: Vec<bool> = vec![false; 26];
            grid.iter().for_each(|row| {
                row.iter().for_each(|ch| m[(ch - b'a') as usize] = true);
            });
            m.iter().filter(|it| **it).count()
        })
        .sum()
}

pub fn part_two(input: &str) -> usize {
    parse_input(input)
        .iter()
        .map(|grid| {
            let n = grid.len();
            let mut m = vec![0; 26];
            grid.iter().for_each(|row| {
                row.iter().for_each(|ch| {
                    let i = (ch - b'a') as usize;
                    m[i] += 1;
                });
            });
            m.into_iter().filter(|it| *it == n).count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(6);
        assert_eq!(part_one(&input), 11);
        assert_eq!(part_two(&input), 6);
    }
}
