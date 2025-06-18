use std::collections::{HashSet, VecDeque};

/// Day 22: Crab Combat - Card game simulation with regular and recursive variants
/// Parse the input into two player decks
fn parse_decks(input: &str) -> (VecDeque<u32>, VecDeque<u32>) {
    let sections: Vec<&str> = input.trim().split("\n\n").collect();

    let parse_deck = |section: &str| -> VecDeque<u32> {
        section
            .lines()
            .skip(1) // Skip "Player X:" line
            .map(|line| line.parse().unwrap())
            .collect()
    };

    (parse_deck(sections[0]), parse_deck(sections[1]))
}

/// Calculate the score of a deck
/// Score = sum of (card_value * position_from_bottom)
fn calculate_score(deck: &VecDeque<u32>) -> usize {
    deck.iter()
        .rev()
        .enumerate()
        .map(|(i, &card)| (i + 1) * card as usize)
        .sum()
}

/// Play regular Combat (Part 1)
/// Players draw top cards, higher card wins both cards
/// Winner places their card first, then the losing card
/// Game continues until one player has all cards
fn play_combat(mut deck1: VecDeque<u32>, mut deck2: VecDeque<u32>) -> usize {
    while !deck1.is_empty() && !deck2.is_empty() {
        let card1 = deck1.pop_front().unwrap();
        let card2 = deck2.pop_front().unwrap();

        if card1 > card2 {
            deck1.push_back(card1);
            deck1.push_back(card2);
        } else {
            deck2.push_back(card2);
            deck2.push_back(card1);
        }
    }

    if deck1.is_empty() {
        calculate_score(&deck2)
    } else {
        calculate_score(&deck1)
    }
}

/// Play Recursive Combat (Part 2)
/// Similar to regular combat but with recursive sub-games
/// If both players have at least as many cards as their drawn card values,
/// the winner is determined by a recursive sub-game
/// Includes infinite game prevention via state tracking
/// Returns (winner, winning_deck) where winner is 1 or 2
fn play_recursive_combat(
    mut deck1: VecDeque<u32>,
    mut deck2: VecDeque<u32>,
) -> (u32, VecDeque<u32>) {
    let mut seen_states: HashSet<(VecDeque<u32>, VecDeque<u32>)> =
        HashSet::new();

    while !deck1.is_empty() && !deck2.is_empty() {
        // Check for infinite game prevention
        let state = (deck1.clone(), deck2.clone());
        if seen_states.contains(&state) {
            // Player 1 wins automatically
            return (1, deck1);
        }
        seen_states.insert(state);

        let card1 = deck1.pop_front().unwrap();
        let card2 = deck2.pop_front().unwrap();

        let player1_wins = if deck1.len() >= card1 as usize
            && deck2.len() >= card2 as usize
        {
            // Recursive sub-game
            let sub_deck1: VecDeque<u32> =
                deck1.iter().take(card1 as usize).copied().collect();
            let sub_deck2: VecDeque<u32> =
                deck2.iter().take(card2 as usize).copied().collect();

            let (winner, _) = play_recursive_combat(sub_deck1, sub_deck2);
            winner == 1
        } else {
            // Regular comparison
            card1 > card2
        };

        if player1_wins {
            deck1.push_back(card1);
            deck1.push_back(card2);
        } else {
            deck2.push_back(card2);
            deck2.push_back(card1);
        }
    }

    if deck1.is_empty() {
        (2, deck2)
    } else {
        (1, deck1)
    }
}

/// Part 1: Play regular Combat and return winning score
/// Simple card game where higher card wins both cards
pub fn part_one(input: &str) -> usize {
    let (deck1, deck2) = parse_decks(input);
    play_combat(deck1, deck2)
}

/// Part 2: Play Recursive Combat and return winning score
/// Complex variant with recursive sub-games when conditions are met
pub fn part_two(input: &str) -> usize {
    let (deck1, deck2) = parse_decks(input);
    let (_, winning_deck) = play_recursive_combat(deck1, deck2);
    calculate_score(&winning_deck)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(22);
        assert_eq!(part_one(&input), 306);
        assert_eq!(part_two(&input), 291);
    }
}
