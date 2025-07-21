//! Day 11: Seating System
//!
//! ## Problem Description
//!
//! Part 1: Apply seating rules based on adjacent seats until stable, count occupied seats.
//! Part 2: Apply seating rules based on visible seats in all directions until stable.
//!
//! ## Solution Approach
//!
//! **Input Parsing**: Converts the seating layout into a 2D grid of characters
//! where 'L'=empty seat, '#'=occupied seat, '.'=floor space.
//!
//! **Part 1 Strategy**: Adjacent seat rules
//! - Empty seat ('L') becomes occupied ('#') if no adjacent occupied seats
//! - Occupied seat ('#') becomes empty ('L') if 4+ adjacent seats occupied
//! - Adjacent = 8 immediate neighbors (including diagonals)
//!
//! **Part 2 Strategy**: Visible seat rules
//! - Empty seat ('L') becomes occupied ('#') if no visible occupied seats
//! - Occupied seat ('#') becomes empty ('L') if 5+ visible seats occupied
//! - Visible = first seat in each of 8 directions (including diagonals)
//!
//! **Simulation**: Uses iterative approach:
//! - Apply rules to entire grid simultaneously (using clone for state)
//! - Continue until no changes occur (stable state)
//! - Count total occupied seats in final configuration
//!
//! **Direction Handling**: 8-directional checking with ray casting for Part 2.

const DIRS: [(i8, i8); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .lines()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect()
}

fn adjacent_occupied(seats: &[Vec<char>], x: i8, y: i8) -> usize {
    let h = seats.len() as i8;
    let w = seats[0].len() as i8;
    DIRS.iter()
        .map(|(dx, dy)| (x + dx, y + dy))
        .filter(|&(x, y)| x >= 0 && x < w && y >= 0 && y < h)
        .map(|(x, y)| (x as usize, y as usize))
        .filter(|&(x, y)| seats[y][x] == '#')
        .count()
}

fn direction_occupied(seats: &[Vec<char>], x: i8, y: i8) -> usize {
    let h = seats.len() as i8;
    let w = seats[0].len() as i8;
    DIRS.iter()
        .map(|(dx, dy)| {
            let (mut x, mut y) = (x, y);
            loop {
                x += dx;
                y += dy;
                if x < 0 || x >= w || y < 0 || y >= h {
                    break false;
                }
                match seats[y as usize][x as usize] {
                    '#' => break true,
                    'L' => break false,
                    _ => continue,
                }
            }
        })
        .filter(|occupied| *occupied)
        .count()
}

fn take_seats(
    seats: &mut Vec<Vec<char>>,
    threshold: usize,
    occupied: fn(&[Vec<char>], i8, i8) -> usize,
) -> bool {
    let origin = seats.clone();
    seats.iter_mut().enumerate().for_each(|(y, row)| {
        row.iter_mut().enumerate().for_each(|(x, seat)| {
            match *seat {
                'L' if occupied(&origin, x as i8, y as i8) == 0 => {
                    *seat = '#';
                }
                '#' if occupied(&origin, x as i8, y as i8) >= threshold => {
                    *seat = 'L';
                }
                _ => {}
            };
        })
    });
    seats != &origin
}

pub fn part_one(input: &str) -> usize {
    let mut seats = parse_input(input);
    while take_seats(&mut seats, 4, adjacent_occupied) {}
    seats
        .iter()
        .map(|row| row.iter().filter(|&&c| c == '#').count())
        .sum()
}

pub fn part_two(input: &str) -> usize {
    let mut seats = parse_input(input);
    while take_seats(&mut seats, 5, direction_occupied) {}
    seats
        .iter()
        .map(|row| row.iter().filter(|&&c| c == '#').count())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(11);
        assert_eq!(part_one(&input), 37);
        assert_eq!(part_two(&input), 26);
    }
}
