fn parse_input(input: &str) -> Vec<i32> {
    input.trim().lines().map(|s| s.parse().unwrap()).collect()
}

pub fn part_one(input: &str) -> usize {
    let mut input = parse_input(input);
    input.push(0);
    input.sort_unstable();
    input.push(input.last().unwrap() + 3);
    let diffs: Vec<i32> = input.windows(2).map(|v| v[1] - v[0]).collect();
    let c1 = diffs.iter().filter(|&v| *v == 1).count();
    let c3 = diffs.iter().filter(|&v| *v == 3).count();
    c1 * c3
}

pub fn part_two(input: &str) -> usize {
    let mut input = parse_input(input);
    input.push(0);
    input.sort_unstable();
    input.push(input.last().unwrap() + 3);

    let n = input.len();
    let mut dp = vec![0; n];
    dp[0] = 1;
    for i in 1..n {
        for j in (0..i).rev() {
            if input[i] - input[j] <= 3 {
                dp[i] += dp[j];
            } else {
                break;
            }
        }
    }
    dp[n - 1]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(10);
        assert_eq!(part_one(&input), 220);
        assert_eq!(part_two(&input), 19208);
    }
}
