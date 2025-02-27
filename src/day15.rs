fn parse_input(input: &str) -> Vec<usize> {
    input
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect()
}

fn target_number(numbers: Vec<usize>, target: usize) -> usize {
    let n = numbers.len();
    // Use a Vec instead of HashMap for better performance
    // Since we're dealing with the last spoken number -> (turn last spoken, current turn)
    let mut visited = vec![None; target];

    // Initialize with starting numbers
    for (i, &num) in numbers.iter().take(n - 1).enumerate() {
        visited[num] = Some((0, i + 1));
    }

    let mut last = numbers[n - 1];
    for i in numbers.len()..target {
        if let Some(prev) = visited.get_mut(last).and_then(|v| v.as_mut()) {
            let j = prev.1;
            *prev = (j, i);
            last = i - j;
        } else {
            visited[last] = Some((0, i));
            last = 0;
        }
    }
    last
}

pub fn part_one(input: &str) -> usize {
    let numbers = parse_input(input);
    target_number(numbers, 2020)
}

pub fn part_two(input: &str) -> usize {
    let numbers = parse_input(input);
    target_number(numbers, 30000000)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(15);
        assert_eq!(part_one(&input), 436);
        assert_eq!(part_two(&input), 175594);
    }

    #[test]
    fn more_examples() {
        for (input, expected) in vec![
            ("1,3,2", 1),
            ("2,1,3", 10),
            ("1,2,3", 27),
            ("2,3,1", 78),
            ("3,2,1", 438),
            ("3,1,2", 1836),
        ] {
            assert_eq!(part_one(input), expected);
        }
    }

    #[test]
    fn more_examples_part_two() {
        for (input, expected) in vec![
            ("1,3,2", 2578),
            ("2,1,3", 3544142),
            ("1,2,3", 261214),
            ("2,3,1", 6895259),
            ("3,2,1", 18),
            ("3,1,2", 362),
        ] {
            assert_eq!(part_two(input), expected);
        }
    }
}
