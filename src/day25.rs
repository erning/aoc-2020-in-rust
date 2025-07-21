//! Day 25: Combo Breaker - Cryptographic key exchange using modular exponentiation
//!
//! Problem Summary:
//! A card and door use a cryptographic key exchange based on modular exponentiation:
//! - Both use subject number 7 with secret loop sizes to generate public keys
//! - Card has secret loop size, publishes public key
//! - Door has secret loop size, publishes public key
//! - Encryption key = transform(door_public_key, card_loop_size) = transform(card_public_key, door_loop_size)
//!
//! Part 1 - Find Encryption Key:
//! - Given two public keys (card and door)
//! - Find the loop size for one device by brute force
//! - Use that loop size to transform the other device's public key
//! - Return the resulting encryption key
//!
//! Solution Approach:
//! - Use modular exponentiation: value = (value * subject) % 20201227
//! - Brute force loop size finding by iterating from subject 7
//! - Transform function applies modular exponentiation 'loop_size' times
//! - The encryption key is symmetric: either transformation yields same result
//!
//! Note: Day 25 traditionally only has Part 1 as the final puzzle

const MODULUS: u64 = 20201227;
const SUBJECT_NUMBER: u64 = 7;

/// Transform a subject number with given loop size using the cryptographic formula
fn transform(subject_number: u64, loop_size: usize) -> u64 {
    let mut value = 1;
    for _ in 0..loop_size {
        value = (value * subject_number) % MODULUS;
    }
    value
}

/// Find the loop size that produces the given public key when transforming subject 7
fn find_loop_size(public_key: u64) -> usize {
    let mut value = 1;
    let mut loop_size = 0;

    loop {
        if value == public_key {
            return loop_size;
        }
        value = (value * SUBJECT_NUMBER) % MODULUS;
        loop_size += 1;
    }
}

/// Parse the input to get the two public keys
fn parse_input(input: &str) -> (u64, u64) {
    let lines: Vec<&str> = input.trim().lines().collect();
    let card_public_key = lines[0].parse().unwrap();
    let door_public_key = lines[1].parse().unwrap();
    (card_public_key, door_public_key)
}

/// Part 1: Calculate the encryption key from the two public keys
pub fn part_one(input: &str) -> u64 {
    let (card_public_key, door_public_key) = parse_input(input);

    // Find the loop size for the card by brute force
    let card_loop_size = find_loop_size(card_public_key);

    // Use card's loop size to transform door's public key to get encryption key
    transform(door_public_key, card_loop_size)
}

/// Part 2: Not applicable for Day 25 (final day traditionally has only one part)
pub fn part_two(_input: &str) -> String {
    "Done".to_string() // Day 25 typically only has Part 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(25);
        assert_eq!(part_one(&input), 14897079);
    }

    #[test]
    fn test_transform() {
        // Test examples from problem description
        assert_eq!(transform(7, 8), 5764801); // Card's public key
        assert_eq!(transform(7, 11), 17807724); // Door's public key

        // Test encryption key calculation both ways
        assert_eq!(transform(17807724, 8), 14897079); // Using card's loop size
        assert_eq!(transform(5764801, 11), 14897079); // Using door's loop size
    }

    #[test]
    fn test_find_loop_size() {
        assert_eq!(find_loop_size(5764801), 8); // Card's loop size
        assert_eq!(find_loop_size(17807724), 11); // Door's loop size
    }
}
