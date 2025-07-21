//! Day 17: Conway Cubes
//!
//! ## Problem Description
//!
//! Part 1: Simulate Conway's Game of Life in 3D space for 6 cycles.
//! Part 2: Simulate Conway's Game of Life in 4D space for 6 cycles.
//!
//! ## Solution Approach
//!
//! **Input Parsing**: Converts 2D input grid into initial cube positions,
//! mapping '#' to active cubes at z=0 (Part 1) or z=w=0 (Part 2).
//!
//! **Part 1 Strategy**: 3D cellular automaton
//! - Active cube stays active with 2-3 active neighbors
//! - Inactive cube becomes active with exactly 3 active neighbors
//! - Uses 3D coordinates (x,y,z) for cube positions
//!
//! **Part 2 Strategy**: 4D cellular automaton
//! - Same rules as Part 1 but in 4D space (x,y,z,w)
//! - Expands simulation bounds each cycle to include all possible neighbors
//!
//! **Simulation Algorithm**:
//! - Uses HashSet to efficiently store only active cube positions
//! - For each cycle, examines all positions within current bounds + 1
//! - Counts active neighbors using nested loops over 3D/4D space
//! - Applies Conway's rules to determine next state
//!
//! **Performance**: Efficient sparse representation using HashSet,
//! only storing active cubes rather than entire grid.

use std::collections::HashSet;

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|s| s.chars().collect()).collect()
}

pub fn part_one(input: &str) -> usize {
    let grid = parse_input(input);
    let h = grid.len();
    let w = grid[0].len();
    let mut cubes: HashSet<(i32, i32, i32)> = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, &c)| {
                if c == '#' {
                    Some((x as i32, y as i32, 0))
                } else {
                    None
                }
            })
        })
        .collect();

    fn process_cube(
        pos: (i32, i32, i32),
        cube: &HashSet<(i32, i32, i32)>,
        new_cubes: &mut HashSet<(i32, i32, i32)>,
    ) {
        let neighbors = {
            let mut count = 0;
            for x in -1..=1 {
                for y in -1..=1 {
                    for z in -1..=1 {
                        if x == 0 && y == 0 && z == 0 {
                            continue;
                        }
                        if cube.contains(&(pos.0 + x, pos.1 + y, pos.2 + z)) {
                            count += 1;
                        }
                    }
                }
            }
            count
        };
        if neighbors == 3 || (cube.contains(&pos) && neighbors == 2) {
            new_cubes.insert(pos);
        }
    }

    let mut new_cubes = HashSet::new();
    for i in 1..=6 {
        for x in -i..w as i32 + i {
            for y in -i..h as i32 + i {
                for z in -i..=i {
                    process_cube((x, y, z), &cubes, &mut new_cubes);
                }
            }
        }
        cubes = new_cubes;
        new_cubes = HashSet::new();
    }

    cubes.len()
}

pub fn part_two(input: &str) -> usize {
    let grid = parse_input(input);
    let h = grid.len();
    let w = grid[0].len();
    let mut cubes: HashSet<(i32, i32, i32, i32)> = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, &c)| {
                if c == '#' {
                    Some((x as i32, y as i32, 0, 0))
                } else {
                    None
                }
            })
        })
        .collect();

    fn process_cube(
        pos: (i32, i32, i32, i32),
        cube: &HashSet<(i32, i32, i32, i32)>,
        new_cubes: &mut HashSet<(i32, i32, i32, i32)>,
    ) {
        let neighbors = {
            let mut count = 0;
            for x in -1..=1 {
                for y in -1..=1 {
                    for z in -1..=1 {
                        for w in -1..=1 {
                            if x == 0 && y == 0 && z == 0 && w == 0 {
                                continue;
                            }
                            if cube.contains(&(
                                pos.0 + x,
                                pos.1 + y,
                                pos.2 + z,
                                pos.3 + w,
                            )) {
                                count += 1;
                            }
                        }
                    }
                }
            }
            count
        };
        if neighbors == 3 || (cube.contains(&pos) && neighbors == 2) {
            new_cubes.insert(pos);
        }
    }

    let mut new_cubes = HashSet::new();
    for i in 1..=6 {
        for x in -i..w as i32 + i {
            for y in -i..h as i32 + i {
                for z in -i..=i {
                    for w in -i..=i {
                        process_cube((x, y, z, w), &cubes, &mut new_cubes);
                    }
                }
            }
        }
        cubes = new_cubes;
        new_cubes = HashSet::new();
    }

    cubes.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(17);
        assert_eq!(part_one(&input), 112);
        assert_eq!(part_two(&input), 848);
    }
}
