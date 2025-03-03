fn parse_input(
    input: &str,
) -> (Vec<(&str, Vec<(u32, u32)>)>, Vec<u32>, Vec<Vec<u32>>) {
    let sections: Vec<&str> =
        input.trim().split("\n\n").collect::<Vec<&str>>();
    let rules: Vec<(&str, Vec<(u32, u32)>)> = sections[0]
        .trim()
        .lines()
        .map(|s| {
            let parts: Vec<&str> = s.splitn(2, ": ").collect();
            let name = parts[0].trim();
            let bounds = parts[1]
                .trim()
                .split(" or ")
                .map(|range| {
                    let bounds: Vec<&str> = range.split('-').collect();
                    (bounds[0].parse().unwrap(), bounds[1].parse().unwrap())
                })
                .collect();
            (name, bounds)
        })
        .collect();

    let ticket: Vec<u32> = sections[1]
        .lines()
        .skip(1)
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let nearby_tickets: Vec<Vec<u32>> = sections[2]
        .lines()
        .skip(1)
        .map(|s| s.split(',').map(|s| s.parse().unwrap()).collect())
        .collect();

    (rules, ticket, nearby_tickets)
}

pub fn part_one(input: &str) -> u32 {
    let (rules, _, nearby_tickets) = parse_input(input);

    let is_invalid = |value: u32| -> bool {
        rules.iter().all(|(_, ranges)| {
            ranges.iter().all(|&(min, max)| value < min || value > max)
        })
    };

    nearby_tickets
        .iter()
        .flat_map(|ticket| ticket.iter().filter(|&value| is_invalid(*value)))
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
        let input = read_example(16);
        assert_eq!(part_one(&input), 71);
    }
}

#[test]
fn example_part_two() {
    let input = concat!(
        "class: 0-1 or 4-19\n",
        "row: 0-5 or 8-19\n",
        "seat: 0-13 or 16-19\n",
        "\n",
        "your ticket:\n",
        "11,12,13\n",
        "\n",
        "nearby tickets:\n",
        "3,9,18\n",
        "15,1,5\n",
        "5,14,9"
    );
}
