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
