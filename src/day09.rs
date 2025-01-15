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
    let invalid = find_invalid(&nums, len);
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
    find_invalid(&nums, 25)
}

pub fn part_two(input: &str) -> u64 {
    let nums = parse_input(input);
    find_invalid_sum(&nums, 25)
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
