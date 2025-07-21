//! Day 16: Ticket Translation
//!
//! ## Problem Description
//!
//! Part 1: Find the sum of all invalid values in nearby tickets based on field rules.
//! Part 2: Determine which fields are which on your ticket and multiply the departure values.
//!
//! ## Solution Approach
//!
//! **Input Parsing**: Parses three sections:
//! - Field rules: name + valid ranges (e.g., "class: 1-3 or 5-7")
//! - Your ticket: comma-separated values
//! - Nearby tickets: list of comma-separated value tickets
//!
//! **Part 1 Strategy**: Invalid value identification
//! - For each value in nearby tickets, check if it matches any field's valid ranges
//! - Sum all values that don't match any field's constraints
//!
//! **Part 2 Strategy**: Field mapping via constraint satisfaction
//! - Filter out invalid tickets using Part 1 criteria
//! - For each ticket position, determine which fields could validly map to it
//! - Use greedy algorithm: assign fields to positions with fewest valid options first
//! - Extract departure-related fields from your ticket and multiply their values
//!
//! **Algorithm**: Uses binary heap for efficient constraint satisfaction with smallest-domain-first heuristic.

use std::cmp::Reverse;
use std::collections::BinaryHeap;

type Ranges = Vec<(u64, u64)>;
type Rule<'a> = (&'a str, Ranges);
type Ticket = Vec<u64>;
type Tickets = Vec<Ticket>;

fn parse_input(input: &str) -> (Vec<Rule>, Ticket, Tickets) {
    let sections: Vec<&str> =
        input.trim().split("\n\n").collect::<Vec<&str>>();
    let rules: Vec<Rule> = sections[0]
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

    let ticket: Vec<u64> = sections[1]
        .lines()
        .nth(1)
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let nearby_tickets: Vec<Vec<u64>> = sections[2]
        .lines()
        .skip(1)
        .map(|s| s.split(',').map(|s| s.parse().unwrap()).collect())
        .collect();

    (rules, ticket, nearby_tickets)
}

pub fn part_one(input: &str) -> u64 {
    let (rules, _, nearby_tickets) = parse_input(input);
    let is_invalid = |value: u64| -> bool {
        rules.iter().all(|(_, ranges)| {
            ranges.iter().all(|&(min, max)| value < min || value > max)
        })
    };
    nearby_tickets
        .iter()
        .flat_map(|ticket| ticket.iter().filter(|&value| is_invalid(*value)))
        .sum()
}

fn determined_ticket_fields(input: &str) -> Vec<(&str, u64)> {
    let (rules, ticket, nearby_tickets) = parse_input(input);

    let tickets: Vec<Vec<u64>> = nearby_tickets
        .into_iter()
        .filter(|ticket| {
            ticket.iter().all(|&value| {
                rules.iter().any(|(_, ranges)| {
                    ranges
                        .iter()
                        .any(|&(min, max)| value >= min && value <= max)
                })
            })
        })
        .collect();

    let valids: Vec<Vec<Vec<bool>>> = tickets
        .iter()
        .map(|ticket| {
            ticket
                .iter()
                .map(|value| {
                    rules
                        .iter()
                        .map(|(_, ranges)| {
                            ranges.iter().any(|&(min, max)| {
                                *value >= min && *value <= max
                            })
                        })
                        .collect()
                })
                .collect()
        })
        .collect();

    let h = valids.len();
    let w = valids[0].len();
    let valid_fields: Vec<Vec<usize>> = (0..w)
        .map(|x| {
            (0..rules.len())
                .filter(|i| (0..h).all(|y| valids[y][x][*i]))
                .collect()
        })
        .collect();

    let mut queue = BinaryHeap::new();
    let mut visited = vec![false; valid_fields.len()];

    for (i, fields) in valid_fields.iter().enumerate() {
        queue.push(Reverse((fields.len(), i, fields)));
    }

    let mut ticket_fields = Vec::new();
    while let Some(Reverse((_, i, fields))) = queue.pop() {
        let v = fields.iter().find(|&&v| !visited[v]).unwrap();
        visited[*v] = true;
        ticket_fields.push((rules[*v].0, ticket[i]));
    }
    ticket_fields
}

pub fn part_two(input: &str) -> u64 {
    determined_ticket_fields(input)
        .iter()
        .filter(|(s, _)| s.starts_with("departure"))
        .map(|(_, v)| *v)
        .product()
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

    let fields = determined_ticket_fields(&input);
    assert!(fields
        .iter()
        .find(|(n, v)| n == &"class" && v == &12)
        .is_some());
    assert!(fields
        .iter()
        .find(|(n, v)| n == &"row" && v == &11)
        .is_some());
    assert!(fields
        .iter()
        .find(|(n, v)| n == &"seat" && v == &13)
        .is_some());
}
