fn parse_input(input: &str) -> Vec<i32> {
    input.trim().lines().map(|s| s.parse().unwrap()).collect()
}

pub fn part_one(input: &str) -> i32 {
    let numbers = parse_input(input);
    let n = numbers.len();
    for (i, a) in numbers.iter().take(n - 1).enumerate() {
        for b in numbers.iter().skip(i) {
            if a + b == 2020 {
                return a * b;
            }
        }
    }
    panic!()
}

pub fn part_two(input: &str) -> i32 {
    let numbers = parse_input(input);
    let n = numbers.len();
    for (i, a) in numbers.iter().enumerate().take(n - 2) {
        for (j, b) in numbers.iter().enumerate().take(n - 1).skip(i) {
            for c in numbers.iter().skip(j) {
                if a + b + c == 2020 {
                    return a * b * c;
                }
            }
        }
    }
    panic!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(1);
        assert_eq!(part_one(&input), 514579);
        assert_eq!(part_two(&input), 241861950);
    }
}
