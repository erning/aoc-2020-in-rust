use std::collections::HashMap;

fn parse_input(input: &str) -> HashMap<String, HashMap<String, usize>> {
    input
        .trim()
        .lines()
        .map(|s| {
            let v: Vec<&str> =
                s.splitn(2, "contain").map(|s| s.trim()).collect();
            let name: String =
                v[0][..v[0].len() - "bags".len()].trim().trim().to_string();
            let contents = v[1]
                .trim()
                .split([','])
                .filter_map(|s| {
                    let v: Vec<&str> = s.split_whitespace().collect();
                    if v.len() < 4 {
                        None
                    } else {
                        let n: usize = v[0].parse().unwrap();
                        let name = v[1..3].join(" ");
                        Some((name, n))
                    }
                })
                .collect();
            (name, contents)
        })
        .collect()
}

fn is_contain_shiny_gold(
    name: &str,
    input: &HashMap<String, HashMap<String, usize>>,
) -> bool {
    if let Some(bag) = input.get(name) {
        bag.contains_key("shiny gold")
            || bag.keys().any(|it| is_contain_shiny_gold(it, input))
    } else {
        false
    }
}

fn contain_bags(
    name: &str,
    input: &HashMap<String, HashMap<String, usize>>,
) -> usize {
    let contents = input.get(name).unwrap();
    if contents.is_empty() {
        0
    } else {
        contents
            .iter()
            .map(|(name, n)| n * contain_bags(name, input) + n)
            .sum()
    }
}

pub fn part_one(input: &str) -> usize {
    let input = parse_input(input);
    input
        .keys()
        .filter(|name| name != &"shiny gold")
        .filter(|name| is_contain_shiny_gold(name, &input))
        .count()
}

pub fn part_two(input: &str) -> usize {
    let input = parse_input(input);
    contain_bags("shiny gold", &input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(7);
        assert_eq!(part_one(&input), 4);
        assert_eq!(part_two(&input), 32);
    }
}
