//! Day 24: Lobby Layout - Hexagonal tile flipping and cellular automaton
//!
//! Problem Summary:
//! A lobby floor has hexagonal tiles arranged in a hexagonal grid. Each tile can be
//! black or white (starts white). Directions are given as paths from a reference tile,
//! and each tile at the end of a path gets flipped (black ↔ white).
//!
//! Part 1 - Initial Configuration:
//! - Parse directional instructions (e.g., "esenee" = east, south-east, north-east, east)
//! - Follow each path from origin (0,0,0) to identify tiles to flip
//! - Count total black tiles after all flips
//!
//! Part 2 - Cellular Automaton:
//! - Apply Conway's Game of Life rules on hexagonal grid for 100 days
//! - Rules:
//!   - Black tile with 1-2 black neighbors stays black
//!   - White tile with exactly 2 black neighbors becomes black
//!   - All other tiles become/remain white
//!
//! Solution Approach:
//! - Use cube coordinates (x,y,z) where x+y+z=0 for hexagonal grid representation
//! - Directions: e(1,-1,0), w(-1,1,0), ne(1,0,-1), nw(0,1,-1), se(0,-1,1), sw(-1,0,1)
//! - Store only black tiles in HashSet<HexCoord> for efficiency
//! - For cellular automaton: count black neighbors for all potentially affected tiles
//! - Parse directions using state machine for multi-character directions (ne, nw, se, sw)

use std::collections::{HashMap, HashSet};

// Hexagonal coordinate using cube coordinates (x, y, z) where x + y + z = 0
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct HexCoord {
    x: i32,
    y: i32,
    z: i32,
}

impl HexCoord {
    fn new(x: i32, y: i32, z: i32) -> Self {
        debug_assert_eq!(x + y + z, 0);
        Self { x, y, z }
    }

    fn origin() -> Self {
        Self::new(0, 0, 0)
    }

    fn move_direction(&self, direction: &str) -> Self {
        let (dx, dy, dz) = match direction {
            "e" => (1, -1, 0),
            "w" => (-1, 1, 0),
            "ne" => (1, 0, -1),
            "nw" => (0, 1, -1),
            "se" => (0, -1, 1),
            "sw" => (-1, 0, 1),
            _ => panic!("Invalid direction: {direction}"),
        };
        Self::new(self.x + dx, self.y + dy, self.z + dz)
    }

    fn neighbors(&self) -> Vec<HexCoord> {
        vec![
            self.move_direction("e"),
            self.move_direction("w"),
            self.move_direction("ne"),
            self.move_direction("nw"),
            self.move_direction("se"),
            self.move_direction("sw"),
        ]
    }
}

/// Parse a line of directional instructions into a sequence of direction strings
fn parse_directions(line: &str) -> Vec<String> {
    let mut directions = Vec::new();
    let mut chars = line.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            'e' | 'w' => directions.push(ch.to_string()),
            'n' | 's' => {
                if let Some(&next_ch) = chars.peek() {
                    if next_ch == 'e' || next_ch == 'w' {
                        directions.push(format!(
                            "{}{}",
                            ch,
                            chars.next().unwrap()
                        ));
                    } else {
                        panic!("Invalid direction starting with {ch}");
                    }
                } else {
                    panic!("Incomplete direction starting with {ch}");
                }
            }
            _ => panic!("Invalid character in directions: {ch}"),
        }
    }

    directions
}

/// Follow directions from origin and return the target coordinate
fn follow_directions(directions: &[String]) -> HexCoord {
    let mut coord = HexCoord::origin();
    for direction in directions {
        coord = coord.move_direction(direction);
    }
    coord
}

/// Parse input and return set of black tiles after initial flipping
fn get_initial_black_tiles(input: &str) -> HashSet<HexCoord> {
    let mut black_tiles = HashSet::new();

    for line in input.lines() {
        let directions = parse_directions(line.trim());
        let target = follow_directions(&directions);

        if black_tiles.contains(&target) {
            black_tiles.remove(&target); // flip back to white
        } else {
            black_tiles.insert(target); // flip to black
        }
    }

    black_tiles
}

/// Simulate one day of the cellular automaton
fn simulate_day(black_tiles: &HashSet<HexCoord>) -> HashSet<HexCoord> {
    let mut neighbor_counts: HashMap<HexCoord, usize> = HashMap::new();

    // Count black neighbors for all tiles that might be affected
    for &tile in black_tiles {
        for neighbor in tile.neighbors() {
            *neighbor_counts.entry(neighbor).or_insert(0) += 1;
        }
    }

    let mut new_black_tiles = HashSet::new();

    // Check all tiles that might change state
    for (&tile, &black_neighbor_count) in &neighbor_counts {
        let is_currently_black = black_tiles.contains(&tile);

        if is_currently_black {
            // Black tile stays black if it has 1 or 2 black neighbors
            if black_neighbor_count == 1 || black_neighbor_count == 2 {
                new_black_tiles.insert(tile);
            }
            // Otherwise it flips to white (not inserted)
        } else {
            // White tile flips to black if it has exactly 2 black neighbors
            if black_neighbor_count == 2 {
                new_black_tiles.insert(tile);
            }
            // Otherwise it stays white (not inserted)
        }
    }

    new_black_tiles
}

/// Part 1: Count black tiles after initial flipping
pub fn part_one(input: &str) -> usize {
    let black_tiles = get_initial_black_tiles(input);
    black_tiles.len()
}

/// Part 2: Count black tiles after 100 days of cellular automaton
pub fn part_two(input: &str) -> usize {
    let mut black_tiles = get_initial_black_tiles(input);

    for _ in 0..100 {
        black_tiles = simulate_day(&black_tiles);
    }

    black_tiles.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(24);
        assert_eq!(part_one(&input), 10);
        assert_eq!(part_two(&input), 2208);
    }

    #[test]
    fn test_hex_coord() {
        let origin = HexCoord::origin();
        assert_eq!(origin.x + origin.y + origin.z, 0);

        let east = origin.move_direction("e");
        assert_eq!(east, HexCoord::new(1, -1, 0));

        let west = origin.move_direction("w");
        assert_eq!(west, HexCoord::new(-1, 1, 0));
    }

    #[test]
    fn test_parse_directions() {
        let directions = parse_directions("esenee");
        assert_eq!(directions, vec!["e", "se", "ne", "e"]);

        let directions = parse_directions("esew");
        assert_eq!(directions, vec!["e", "se", "w"]);

        let directions = parse_directions("nwwswee");
        assert_eq!(directions, vec!["nw", "w", "sw", "e", "e"]);
    }
}
