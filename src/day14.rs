//! Day 14: Docking Data
//!
//! ## Problem Description
//!
//! Part 1: Apply bitmask operations to memory values before storing them.
//! Part 2: Apply bitmask operations to memory addresses, with floating bits creating
//! multiple address variants.
//!
//! ## Solution Approach
//!
//! **Input Parsing**: Splits input into sections by "mask = " delimiter:
//! - Each section starts with a bitmask (36 characters of 0, 1, X)
//! - Followed by memory assignments in format "mem[address] = value"
//!
//! **Part 1 Strategy**: Value masking
//! - Parse bitmask into two bitmasks: bm0 (for 0s) and bm1 (for 1s)
//! - Apply masks: (value | bm1) & !bm0
//! - bm1 forces 1s, bm0 forces 0s, X bits remain unchanged
//!
//! **Part 2 Strategy**: Address masking with floating bits
//! - Parse bitmask to identify: fixed 1s (bm1), fixed 0s (bm0), and floating bits (X)
//! - For each memory assignment, generate all possible addresses by setting floating bits
//! - Uses recursive generation to handle all 2^n combinations for n floating bits
//! - Stores value in all generated addresses
//!
//! **Bit Manipulation**: Uses bitwise operations for efficient mask application and
//! recursive address generation for floating bits.

use std::collections::HashMap;

fn parse_input(input: &str) -> Vec<Vec<(u64, u64)>> {
    input
        .split("mask = ")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|section| {
            section
                .lines()
                .map(|s| {
                    if let Some(s) = s.strip_prefix("mem[") {
                        let parts: Vec<&str> = s.splitn(2, "] = ").collect();
                        (parts[0].parse().unwrap(), parts[1].parse().unwrap())
                    } else {
                        let (mut bm0, mut bm1) = (0, 0);
                        for c in s.chars() {
                            bm0 <<= 1;
                            bm1 <<= 1;
                            match c {
                                '0' => bm0 |= 1,
                                '1' => bm1 |= 1,
                                'X' => {}
                                _ => unreachable!(),
                            }
                        }
                        (bm0, bm1)
                    }
                })
                .collect()
        })
        .collect()
}

pub fn part_one(input: &str) -> u64 {
    let program = parse_input(input);
    let mut memory = HashMap::<u64, u64>::new();
    for section in program.iter() {
        let (bm0, bm1) = section[0];
        for &(address, value) in section[1..].iter() {
            memory.insert(address, (value | bm1) & !bm0);
        }
    }
    memory.values().sum()
}

pub fn part_two(input: &str) -> u64 {
    let program = parse_input(input);
    let mut memory = HashMap::<u64, u64>::new();
    for section in program.iter() {
        let (bm0, bm1) = section[0];
        let bmx = !bm0 & !bm1 & 0b111111111111111111111111111111111111;
        let bits: Vec<u8> = (0..36).filter(|i| bmx & (1 << i) != 0).collect();
        fn setbmx(
            memory: &mut HashMap<u64, u64>,
            address: u64,
            value: u64,
            bits: &[u8],
        ) {
            if let Some(shift) = bits.first() {
                let mask = 1 << shift;
                for addr in [address & !mask, address | mask] {
                    memory.insert(addr, value);
                    setbmx(memory, addr, value, &bits[1..]);
                }
            }
        }
        for &(address, value) in section[1..].iter() {
            setbmx(&mut memory, address | bm1 & !bmx, value, &bits);
        }
    }
    memory.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(14);
        assert_eq!(part_one(&input), 165);
    }

    #[test]
    fn example_part_two() {
        let input = concat!(
            "mask = 000000000000000000000000000000X1001X\n",
            "mem[42] = 100\n",
            "mask = 00000000000000000000000000000000X0XX\n",
            "mem[26] = 1\n"
        );
        assert_eq!(part_two(input), 208);

        let input = concat!(
            "mask = 000000000000000000000000000000X1001X\n",
            "mem[42] = 100\n",
            "mask = 00000000000000000000000000000000X0XX\n",
            "mem[30] = 1\n"
        );
        assert_eq!(part_two(input), 1 * 8 + 100 * 4);
    }
}
