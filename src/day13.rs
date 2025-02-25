fn parse_input(input: &str) -> (usize, Vec<usize>) {
    let lines: Vec<&str> = input.trim().lines().collect();
    (
        lines[0].parse().unwrap(),
        lines[1]
            .split(',')
            .map(|s| s.parse().unwrap_or(0))
            .collect(),
    )
}

pub fn part_one(input: &str) -> usize {
    let (earliest_departure, bus_ids) = parse_input(input);
    let mut min_wait = usize::MAX;
    let mut min_id = 0;
    for id in bus_ids {
        if id == 0 {
            continue;
        }
        let wait = id - earliest_departure % id;
        if wait < min_wait {
            min_wait = wait;
            min_id = id;
        }
    }
    min_id * min_wait
}

pub fn part_two(input: &str) -> usize {
    let (_, bus_ids) = parse_input(input);

    let mut timestamp = 0;
    let mut step = 1;
    for (i, id) in bus_ids.iter().enumerate() {
        if *id == 0 {
            continue;
        }
        while (timestamp + i) % *id != 0 {
            timestamp += step;
        }
        step *= *id;
    }
    timestamp
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(13);
        assert_eq!(part_one(&input), 295);
        assert_eq!(part_two(&input), 1068781);
    }

    #[test]
    fn example_others() {
        assert_eq!(part_two("939\n17,x,13,19"), 3417);
        assert_eq!(part_two("939\n67,7,59,61"), 754018);
        assert_eq!(part_two("939\n67,x,7,59,61"), 779210);
        assert_eq!(part_two("939\n67,7,x,59,61"), 1261476);
        assert_eq!(part_two("939\n1789,37,47,1889"), 1202161486);
    }
}
