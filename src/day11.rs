use std::collections::HashMap;

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

fn parse_input(input: &str) -> HashMap<(i8, i8), char> {
    input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, s)| {
            s.chars()
                .enumerate()
                .map(|(x, c)| ((x as i8, y as i8), c))
                .collect::<Vec<_>>()
        })
        .collect()
}

fn adjacent_occupied(seats: &HashMap<(i8, i8), char>, x: i8, y: i8) -> usize {
    DIRS.iter()
        .filter(|(dx, dy)| matches!(seats.get(&(x + dx, y + dy)), Some('#')))
        .count()
}

fn take_seats(seats: &HashMap<(i8, i8), char>) -> HashMap<(i8, i8), char> {
    let mut new_seats = seats.clone();
    new_seats.iter_mut().for_each(|((x, y), seat)| match seat {
        'L' if adjacent_occupied(seats, *x, *y) == 0 => {
            *seat = '#';
        }
        '#' if adjacent_occupied(seats, *x, *y) >= 4 => {
            *seat = 'L';
        }
        _ => {}
    });
    new_seats
}

pub fn part_one(input: &str) -> usize {
    let mut seats = parse_input(input);
    loop {
        let new_seats = take_seats(&seats);
        if new_seats == seats {
            break;
        }
        seats = new_seats;
    }
    seats.values().filter(|&&c| c == '#').count()
}

pub fn part_two(input: &str) -> usize {
    0
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
