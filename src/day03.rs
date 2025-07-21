//! Day 3: Toboggan Trajectory
//!
//! ## Problem Description
//!
//! Part 1: Count trees (#) encountered while sledding down a slope following
//! a specific path: right 3, down 1. The terrain repeats horizontally.
//!
//! Part 2: Multiply the tree counts for multiple slope patterns:
//! right 1 down 1, right 3 down 1, right 5 down 1, right 7 down 1, right 1 down 2.
//!
//! ## Solution Approach
//!
//! **Input Parsing**: Converts each line of the terrain map into a vector of characters,
//! creating a 2D grid representation.
//!
//! **Part 1 Strategy**: Single slope traversal
//! - Start at top-left position (0,0)
//! - Move right 3, down 1 repeatedly
//! - Use modulo arithmetic to handle horizontal repeating pattern
//! - Count '#' characters encountered
//!
//! **Part 2 Strategy**: Multiple slope multiplication
//! - Apply the same traversal logic to 5 different slope patterns
//! - Calculate tree count for each slope individually
//! - Multiply all counts together for final result
//!
//! **Grid Traversal**: The `slope` function handles the core logic:
//! - Takes dx (right movement) and dy (down movement) parameters
//! - Uses modulo on x-coordinate to handle infinite horizontal repetition
//! - Returns tree count for the specified slope pattern

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.trim().lines().map(|s| s.chars().collect()).collect()
}

fn slope(grid: &[Vec<char>], dx: usize, dy: usize) -> usize {
    let h = grid.len();
    let w = grid[0].len();
    let (mut x, mut y) = (0, 0);
    let mut trees = 0;
    while y < h {
        if grid[y][x % w] == '#' {
            trees += 1;
        }
        x += dx;
        y += dy;
    }
    trees
}

pub fn part_one(input: &str) -> usize {
    let grid = parse_input(input);
    slope(&grid, 3, 1)
}

pub fn part_two(input: &str) -> usize {
    let grid = parse_input(input);
    [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .into_iter()
        .map(|(dx, dy)| slope(&grid, dx, dy))
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(3);
        assert_eq!(part_one(&input), 7);
        assert_eq!(part_two(&input), 336);
    }
}
