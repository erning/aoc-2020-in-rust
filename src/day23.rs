use std::collections::VecDeque;

/// Day 23: Crab Cups - Cup shuffling game with circular arrangement
/// Part 1: 100 moves with 9 cups, Part 2: 10M moves with 1M cups
/// Parse the input string into a vector of cup numbers
fn parse_input(input: &str) -> Vec<u32> {
    input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

/// Play the crab cups game for Part 1 (simple version)
/// Uses VecDeque for simplicity but slower for large inputs
/// Each move: pick up 3 cups, find destination, insert after destination, move current
fn play_game_simple(cups: Vec<u32>, moves: usize) -> VecDeque<u32> {
    let mut circle: VecDeque<u32> = cups.into_iter().collect();
    let mut current_index = 0;

    for _ in 0..moves {
        let current_cup = circle[current_index];

        // Pick up 3 cups clockwise from current
        let mut picked_up = Vec::new();
        for _ in 0..3 {
            let pick_index = (current_index + 1) % circle.len();
            picked_up.push(circle.remove(pick_index).unwrap());
            // Adjust current_index if we removed something before it
            if pick_index <= current_index && current_index > 0 {
                current_index -= 1;
            }
        }

        // Find destination cup
        let max_cup = circle.iter().max().unwrap();
        let mut destination = if current_cup == 1 {
            *max_cup
        } else {
            current_cup - 1
        };

        while !circle.contains(&destination) {
            destination = if destination == 1 {
                *max_cup
            } else {
                destination - 1
            };
        }

        // Find destination position and insert picked up cups
        let dest_pos = circle.iter().position(|&x| x == destination).unwrap();
        for (i, &cup) in picked_up.iter().enumerate() {
            circle.insert(dest_pos + 1 + i, cup);
            // Adjust current_index if we inserted before it
            if dest_pos + 1 + i <= current_index {
                current_index += 1;
            }
        }

        // Move current cup clockwise
        current_index = (current_index + 1) % circle.len();
    }

    circle
}

/// Play the crab cups game efficiently using a linked list approach (for Part 2)
/// Uses array-based linked list where next[i] = cup that comes after cup i
/// This allows O(1) insertions and removals, critical for 10M moves with 1M cups
fn play_game_efficient(
    cups: Vec<u32>,
    total_cups: usize,
    moves: usize,
) -> Vec<u32> {
    // Create linked list representation: next[i] = cup that comes after cup i
    let mut next = vec![0; total_cups + 1];

    // Build extended cup list
    let mut all_cups = cups.clone();
    for i in (cups.len() as u32 + 1)..=(total_cups as u32) {
        all_cups.push(i);
    }

    // Set up the circular linked list
    for i in 0..all_cups.len() {
        let current = all_cups[i] as usize;
        let next_cup = all_cups[(i + 1) % all_cups.len()] as usize;
        next[current] = next_cup;
    }

    let mut current = all_cups[0] as usize;

    for _ in 0..moves {
        // Pick up the three cups after current
        let pickup1 = next[current];
        let pickup2 = next[pickup1];
        let pickup3 = next[pickup2];

        // Remove the picked up cups from the circle
        next[current] = next[pickup3];

        // Find destination cup
        let mut destination = if current == 1 {
            total_cups
        } else {
            current - 1
        };
        while destination == pickup1
            || destination == pickup2
            || destination == pickup3
        {
            destination = if destination == 1 {
                total_cups
            } else {
                destination - 1
            };
        }

        // Insert picked up cups after destination
        let after_destination = next[destination];
        next[destination] = pickup1;
        next[pickup3] = after_destination;

        // Move to next current cup
        current = next[current];
    }

    // Reconstruct the circle starting from cup 1
    let mut result = Vec::new();
    let mut cup = next[1]; // Start from the cup after 1
    while cup != 1 {
        result.push(cup as u32);
        cup = next[cup];
    }

    result
}

/// Part 1: Play 100 moves with 9 cups, return order after cup 1
/// Returns concatenated cup labels clockwise from cup 1 (excluding cup 1 itself)
pub fn part_one(input: &str) -> String {
    let cups = parse_input(input);
    let result = play_game_simple(cups, 100);

    // Find cup 1 and return all cups after it in order
    let one_pos = result.iter().position(|&x| x == 1).unwrap();
    let mut answer = String::new();

    for i in 1..result.len() {
        let pos = (one_pos + i) % result.len();
        answer.push_str(&result[pos].to_string());
    }

    answer
}

/// Part 2: Play 10M moves with 1M cups, return product of two cups after cup 1
/// Extends cups 1-9 to 1-1000000, then multiplies the two cups immediately clockwise from cup 1
pub fn part_two(input: &str) -> u64 {
    let cups = parse_input(input);
    let result = play_game_efficient(cups, 1_000_000, 10_000_000);

    // The result already starts after cup 1, so first two elements
    let cup1 = result[0] as u64;
    let cup2 = result[1] as u64;

    cup1 * cup2
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(23);
        assert_eq!(part_one(&input), "67384529");
        assert_eq!(part_two(&input), 149245887792);
    }
}
