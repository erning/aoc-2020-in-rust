//! Day 20: Jurassic Jigsaw
//!
//! ## Problem Description
//!
//! Part 1: Assemble a jigsaw puzzle from square tiles and find the product of corner tile IDs.
//! Part 2: Find sea monsters in the assembled image and count '#' characters not part of any monster.
//!
//! ## Solution Approach
//!
//! **Input Parsing**: Parse input into tiles with:
//! - Tile ID (from "Tile ####:")
//! - 10x10 grid of '#' (active) and '.' (inactive) pixels
//!
//! **Part 1 Strategy**: Edge matching algorithm
//! - Extract all 4 edges (top, right, bottom, left) from each tile
//! - Consider both original and flipped versions of edges
//! - Find tiles with exactly 2 matching neighbors (corners)
//! - Return product of corner tile IDs
//!
//! **Part 2 Strategy**: Image assembly and pattern matching
//! - Assemble tiles into complete image by matching edges
//! - Remove borders from each tile (leaving 8x8 pixels per tile)
//! - Search for sea monster pattern in all orientations (8 total: 4 rotations × 2 flips)
//! - Count total '#' characters minus those part of sea monsters
//!
//! **Tile Operations**:
//! - Rotate 90° clockwise: Transpose and reverse rows
//! - Flip horizontal: Reverse each row
//! - All orientations: 8 possible (4 rotations × 2 flips)
//!
//! **Sea Monster Pattern**:
//! - 3-line pattern with specific '#' positions
//! - Search across entire image in all orientations

use std::collections::{HashMap, HashSet};

/// Represents a square tile in the jigsaw puzzle
#[derive(Debug, Clone)]
struct Tile {
    id: usize,
    data: Vec<String>,
}

impl Tile {
    fn new(id: usize, data: Vec<String>) -> Self {
        Self { id, data }
    }

    // Get the four edges as strings
    fn edges(&self) -> [String; 4] {
        let top = self.data[0].clone();
        let bottom = self.data[self.data.len() - 1].clone();
        let left: String = self
            .data
            .iter()
            .map(|row| row.chars().next().unwrap())
            .collect();
        let right: String = self
            .data
            .iter()
            .map(|row| row.chars().last().unwrap())
            .collect();
        [top, right, bottom, left] // clockwise from top
    }

    // Rotate tile 90 degrees clockwise
    fn rotate(&mut self) {
        let size = self.data.len();
        let mut new_data = vec![String::new(); size];
        for (i, row) in new_data.iter_mut().enumerate() {
            for j in 0..size {
                row.push(self.data[size - 1 - j].chars().nth(i).unwrap());
            }
        }
        self.data = new_data;
    }

    // Flip tile horizontally
    fn flip_horizontal(&mut self) {
        for row in &mut self.data {
            *row = row.chars().rev().collect();
        }
    }

    // Get all possible orientations of this tile
    fn all_orientations(&self) -> Vec<Tile> {
        let mut orientations = Vec::new();
        let mut tile = self.clone();

        // 4 rotations
        for _ in 0..4 {
            orientations.push(tile.clone());
            tile.rotate();
        }

        // Flip and 4 more rotations
        tile.flip_horizontal();
        for _ in 0..4 {
            orientations.push(tile.clone());
            tile.rotate();
        }

        orientations
    }

    // Remove border (for part 2)
    fn remove_border(&self) -> Vec<String> {
        let mut result = Vec::new();
        for i in 1..self.data.len() - 1 {
            let row = &self.data[i];
            result.push(row[1..row.len() - 1].to_string());
        }
        result
    }
}

/// Parse the input string into a vector of tiles
fn parse_tiles(input: &str) -> Vec<Tile> {
    let mut tiles = Vec::new();
    let tile_blocks: Vec<&str> = input.trim().split("\n\n").collect();

    for block in tile_blocks {
        let lines: Vec<&str> = block.lines().collect();
        let id_line = lines[0];
        let id: usize = id_line
            .strip_prefix("Tile ")
            .unwrap()
            .strip_suffix(":")
            .unwrap()
            .parse()
            .unwrap();

        let data: Vec<String> =
            lines[1..].iter().map(|s| s.to_string()).collect();
        tiles.push(Tile::new(id, data));
    }

    tiles
}

/// Find which tiles can connect to each other based on matching edges
fn find_edge_matches(tiles: &[Tile]) -> HashMap<usize, HashSet<usize>> {
    let mut matches: HashMap<usize, HashSet<usize>> = HashMap::new();

    // Get all edges for each tile (including flipped versions)
    let mut all_edges: HashMap<String, Vec<usize>> = HashMap::new();

    for tile in tiles {
        let edges = tile.edges();
        for edge in &edges {
            all_edges.entry(edge.clone()).or_default().push(tile.id);
            // Also add the reversed edge
            let reversed: String = edge.chars().rev().collect();
            all_edges.entry(reversed).or_default().push(tile.id);
        }
    }

    // Find which tiles can connect to each other
    for (_edge, tile_ids) in all_edges {
        if tile_ids.len() >= 2 {
            for &id1 in &tile_ids {
                for &id2 in &tile_ids {
                    if id1 != id2 {
                        matches.entry(id1).or_default().insert(id2);
                    }
                }
            }
        }
    }

    matches
}

/// Part 1: Find the product of corner tile IDs
pub fn part_one(input: &str) -> usize {
    let tiles = parse_tiles(input);
    let matches = find_edge_matches(&tiles);

    // Corner tiles have exactly 2 matching neighbors
    let corner_tiles: Vec<usize> = matches
        .iter()
        .filter(|(_, neighbors)| neighbors.len() == 2)
        .map(|(id, _)| *id)
        .collect();

    corner_tiles.iter().product()
}

/// Assemble the jigsaw puzzle into a complete image
fn assemble_image(tiles: &[Tile]) -> Vec<String> {
    let matches = find_edge_matches(tiles);
    let grid_size = (tiles.len() as f64).sqrt() as usize;

    // Find a corner to start with
    let corner_id = matches
        .iter()
        .find(|(_, neighbors)| neighbors.len() == 2)
        .map(|(id, _)| *id)
        .unwrap();

    let tile_map: HashMap<usize, Tile> =
        tiles.iter().map(|t| (t.id, t.clone())).collect();
    let mut used_tiles: HashSet<usize> = HashSet::new();
    let mut grid: Vec<Vec<Option<Tile>>> =
        vec![vec![None; grid_size]; grid_size];

    // This would need a complex backtracking algorithm to properly solve
    // For now, we'll create a simplified version that works with the test case

    // Place corner tile in top-left, trying different orientations
    let corner_tile = &tile_map[&corner_id];
    for orientation in corner_tile.all_orientations() {
        grid[0][0] = Some(orientation.clone());
        used_tiles.insert(corner_id);

        // Try to solve the rest recursively (simplified)
        if solve_grid(
            &mut grid,
            &tile_map,
            &matches,
            &mut used_tiles,
            0,
            1,
            grid_size,
        ) {
            break;
        }

        grid[0][0] = None;
        used_tiles.remove(&corner_id);
    }

    // Combine tiles into final image (removing borders)
    let mut final_image = Vec::new();
    for (_row, grid_row) in grid.iter().enumerate().take(grid_size) {
        let mut tile_rows = vec![Vec::new(); 8]; // 8x8 after removing borders

        for tile in grid_row.iter().take(grid_size).flatten() {
            let borderless = tile.remove_border();
            for (i, line) in borderless.iter().enumerate() {
                tile_rows[i].push(line.clone());
            }
        }

        for tile_row in tile_rows {
            final_image.push(tile_row.join(""));
        }
    }

    final_image
}

fn solve_grid(
    grid: &mut Vec<Vec<Option<Tile>>>,
    tile_map: &HashMap<usize, Tile>,
    _matches: &HashMap<usize, HashSet<usize>>,
    used_tiles: &mut HashSet<usize>,
    row: usize,
    col: usize,
    grid_size: usize,
) -> bool {
    if row == grid_size {
        return true; // Successfully filled the grid
    }

    let (next_row, next_col) = if col + 1 == grid_size {
        (row + 1, 0)
    } else {
        (row, col + 1)
    };

    // Try each unused tile
    for (&tile_id, tile) in tile_map.iter() {
        if used_tiles.contains(&tile_id) {
            continue;
        }

        // Try each orientation of the tile
        for orientation in tile.all_orientations() {
            if can_place_tile(grid, &orientation, row, col) {
                grid[row][col] = Some(orientation);
                used_tiles.insert(tile_id);

                if solve_grid(
                    grid, tile_map, _matches, used_tiles, next_row, next_col,
                    grid_size,
                ) {
                    return true;
                }

                grid[row][col] = None;
                used_tiles.remove(&tile_id);
            }
        }
    }

    false
}

fn can_place_tile(
    grid: &[Vec<Option<Tile>>],
    tile: &Tile,
    row: usize,
    col: usize,
) -> bool {
    let edges = tile.edges();

    // Check top neighbor
    if row > 0 {
        if let Some(top_tile) = &grid[row - 1][col] {
            let top_edges = top_tile.edges();
            if edges[0] != top_edges[2] {
                // top edge must match bottom edge of top tile
                return false;
            }
        }
    }

    // Check left neighbor
    if col > 0 {
        if let Some(left_tile) = &grid[row][col - 1] {
            let left_edges = left_tile.edges();
            if edges[3] != left_edges[1] {
                // left edge must match right edge of left tile
                return false;
            }
        }
    }

    true
}

/// Find sea monsters in the assembled image and return count of '#' not part of monsters
fn find_sea_monsters(image: &[String]) -> usize {
    let sea_monster = [
        "                  # ",
        "#    ##    ##    ###",
        " #  #  #  #  #  #   ",
    ];

    let monster_positions: Vec<(usize, usize)> = sea_monster
        .iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(col, _)| (row, col))
        })
        .collect();

    let mut image_copy = image.to_vec();
    let mut monsters_found = 0;

    // Try all orientations of the image
    for i in 0..8 {
        monsters_found += mark_monsters(&mut image_copy, &monster_positions);
        if monsters_found > 0 {
            break;
        }

        // Rotate image
        image_copy = rotate_image(&image_copy);
        if monsters_found == 0 && i == 3 {
            // Try flipping after 4 rotations
            image_copy = flip_image(&image_copy);
        }
    }

    // Count remaining # characters
    image_copy
        .iter()
        .map(|line| line.chars().filter(|&c| c == '#').count())
        .sum()
}

fn mark_monsters(
    image: &mut [String],
    monster_positions: &[(usize, usize)],
) -> usize {
    let mut monsters_found = 0;
    let rows = image.len();
    let cols = image[0].len();

    for start_row in 0..rows.saturating_sub(2) {
        for start_col in 0..cols.saturating_sub(19) {
            let mut is_monster = true;

            for &(row_offset, col_offset) in monster_positions {
                let check_row = start_row + row_offset;
                let check_col = start_col + col_offset;

                if image[check_row].chars().nth(check_col).unwrap() != '#' {
                    is_monster = false;
                    break;
                }
            }

            if is_monster {
                monsters_found += 1;
                // Mark the monster positions as 'O'
                for &(row_offset, col_offset) in monster_positions {
                    let mark_row = start_row + row_offset;
                    let mark_col = start_col + col_offset;
                    let mut chars: Vec<char> =
                        image[mark_row].chars().collect();
                    chars[mark_col] = 'O';
                    image[mark_row] = chars.into_iter().collect();
                }
            }
        }
    }

    monsters_found
}

fn rotate_image(image: &[String]) -> Vec<String> {
    let rows = image.len();
    let cols = image[0].len();
    let mut rotated = vec![String::new(); cols];

    for (j, rotated_row) in rotated.iter_mut().enumerate() {
        for i in (0..rows).rev() {
            rotated_row.push(image[i].chars().nth(j).unwrap());
        }
    }

    rotated
}

fn flip_image(image: &[String]) -> Vec<String> {
    image
        .iter()
        .map(|line| line.chars().rev().collect())
        .collect()
}

/// Part 2: Count '#' characters that are not part of sea monsters
pub fn part_two(input: &str) -> usize {
    let tiles = parse_tiles(input);
    let image = assemble_image(&tiles);
    find_sea_monsters(&image)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(20);
        assert_eq!(part_one(&input), 20899048083289);
        assert_eq!(part_two(&input), 273);
    }
}
