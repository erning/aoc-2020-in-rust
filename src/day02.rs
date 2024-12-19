type Policy = (usize, usize, char);

fn parse_input(input: &str) -> Vec<(Policy, &str)> {
    input
        .trim()
        .lines()
        .map(|s| {
            //
            let parts: Vec<&str> =
                s.split(['-', ' ', ':']).map(|s| s.trim()).collect();
            (
                (
                    parts[0].parse().unwrap(),
                    parts[1].parse().unwrap(),
                    parts[2].chars().next().unwrap(),
                ),
                parts[4],
            )
        })
        .collect()
}

pub fn part_one(input: &str) -> usize {
    parse_input(input)
        .iter()
        .filter(|((lo, hi, ch), pwd)| {
            (*lo..=*hi).contains(&pwd.chars().filter(|v| v == ch).count())
        })
        .count()
}

pub fn part_two(input: &str) -> usize {
    parse_input(input)
        .iter()
        .filter(|((lo, hi, ch), pwd)| {
            (pwd.chars().nth(lo - 1) == Some(*ch))
                != (pwd.chars().nth(hi - 1) == Some(*ch))
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(2);
        assert_eq!(part_one(&input), 2);
        assert_eq!(part_two(&input), 1);
    }
}
