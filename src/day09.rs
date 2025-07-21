//! Day 9: Encoding Error
//!
//! ## Problem Description
//!
//! Part 1: Find the first number that is NOT the sum of any two of the previous N numbers.
//! Part 2: Find a contiguous range that sums to the invalid number, then return
//! the sum of the smallest and largest numbers in that range.
//!
//! ## Solution Approach
//!
//! **Input Parsing**: Converts input lines into a vector of unsigned 64-bit integers.
//!
//! **Part 1 Strategy**: XMAS cipher validation
//! - Uses sliding window of previous N numbers (N=5 for example, N=25 for real input)
//! - For each number, checks if it can be expressed as sum of any two distinct numbers
//! - Returns the first number that fails this validation
//!
//! **Part 2 Strategy**: Contiguous sum search
//! - Uses sliding window approach to find contiguous range summing to invalid number
//! - Expands window when sum is too small, shrinks when sum is too large
//! - Once found, returns sum of min and max values in the contiguous range
//!
//! **Window Algorithm**: Efficient O(n) sliding window technique to find contiguous sum.

fn parse_input(input: &str) -> Vec<u64> {
    input.trim().lines().map(|s| s.parse().unwrap()).collect()
}

fn is_valid(nums: &[u64], num: u64) -> bool {
    for a in nums {
        for b in nums {
            if a != b && a + b == num {
                return true;
            }
        }
    }
    false
}

fn find_invalid(nums: &[u64], len: usize) -> u64 {
    let n = nums.len();
    for i in len..n {
        let number = nums[i];
        if !is_valid(&nums[i - len..i], number) {
            return number;
        }
    }
    panic!()
}

fn find_invalid_sum(nums: &[u64], len: usize) -> u64 {
    let invalid = find_invalid(nums, len);
    let mut a = 0;
    let mut b = 1;
    let mut sum = nums[a] + nums[b];
    loop {
        if sum < invalid {
            b += 1;
            assert!(b < nums.len());
            sum += nums[b];
            continue;
        }
        if sum > invalid {
            sum -= nums[a];
            a += 1;
            assert!(a < b);
            continue;
        }
        break;
    }
    let slice = &nums[a..b + 1];
    slice.iter().min().unwrap() + slice.iter().max().unwrap()
}

pub fn part_one(input: &str) -> u64 {
    let nums = parse_input(input);
    let numbers = if nums.len() <= 20 { 5 } else { 25 };
    find_invalid(&nums, numbers)
}

pub fn part_two(input: &str) -> u64 {
    let nums = parse_input(input);
    let numbers = if nums.len() <= 20 { 5 } else { 25 };
    find_invalid_sum(&nums, numbers)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(9);
        let numbers = parse_input(&input);
        assert_eq!(find_invalid(&numbers, 5), 127);
        assert_eq!(find_invalid_sum(&numbers, 5), 62);
    }
}
