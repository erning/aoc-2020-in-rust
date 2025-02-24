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

fn take_seats(seats: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut new_seats = seats.to_vec();
    new_seats.iter_mut().enumerate().for_each(|(y, row)| {
        row.iter_mut().enumerate().for_each(|(x, seat)| {
            match *seat {
                'L' if adjacent_occupied(seats, x as i8, y as i8) == 0 => {
                    *seat = '#';
                }
                '#' if adjacent_occupied(seats, x as i8, y as i8) >= 4 => {
                    *seat = 'L';
                }
                _ => {}
            };
        })
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
    seats
        .iter()
        .map(|row| row.iter().filter(|&&c| c == '#').count())
        .sum()
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
